use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::{JoinHandle, spawn, sleep};
use super::world::{Input, Connection, ClientView};
use std::collections::{HashMap, VecDeque};
use bytes::{BytesMut, BufMut};
use std::ops::{Deref, DerefMut};
use std::io::{Read, Write};
use std::sync::mpsc::{Sender, channel, Receiver};
use std::time::Duration;
use crate::utils::StreamHandler;
use crate::utils::find_stream_end_chars;

pub(crate) type InputBufferMutex = Arc<Mutex<PlayerInputBuffer>>;

struct Message {

}

pub(crate) struct PlayerInputBuffer {
    inner: HashMap<String, VecDeque<Input>>
}

pub struct StreamData {
    pub login_key: String,
    pub should_connect: bool
}


pub(crate) struct Server {
    stream_handle: StreamHandler,
    tcp_listener: TcpListener,
    input_stream: InputBufferMutex,
    connection_channel: Sender<(Connection, Sender<ClientView>)>,
}

#[derive(Clone)]
pub(crate) struct ServerConfig {
    pub port: u16,
    pub server_name: String
}

impl PlayerInputBuffer {
    pub fn new() -> Self {
        PlayerInputBuffer {
            inner: HashMap::new()
        }
    }

    pub fn push_input(&mut self, player: String, input: Input) {
        if let Some(mut input_v) = self.inner.get_mut(&player) {
            input_v.push_back(input);
        } else {
            self.inner.insert(player, cascade::cascade! {
                VecDeque::new();
                ..push_back(input);
            });
        }
    }

    pub fn pop_input(&mut self, player: String) -> Option<Input> {
        if let Some(mut input_v) = self.inner.get_mut(&player) {
            input_v.pop_front()
        } else {
            None
        }
    }
}

impl ServerConfig {
    pub fn new() -> Self {
        ServerConfig {
            port: 1212, // the default port for Hyperspeed
            server_name: "default_name".to_string()
        }
    }
}

impl Server {
    pub(crate) fn new(s: ServerConfig, c_sender: Sender<(Connection, Sender<ClientView>)>, stream_handler: StreamHandler) -> Server {
        Server {
            tcp_listener: TcpListener::bind(format!("0.0.0.0:{}", s.port)).unwrap(),
            input_stream: Arc::new(Mutex::new(PlayerInputBuffer::new())),
            connection_channel: c_sender,
            stream_handle: stream_handler
        }
    }
    pub(crate) fn main_loop(&mut self) {
        for stream in self.tcp_listener.incoming() {
            let mut stream = stream.unwrap();
            let data = (self.stream_handle)(&mut stream);
            match data {
                StreamData {
                    login_key,
                    should_connect
                } => {
                    if should_connect {
                        let mutex_clone = self.input_stream.clone();
                        let (send, recv) = channel();
                        let conn = Connection { key: login_key.clone() };
                        self.connection_channel.send((conn, send));
                        spawn(move || stream_communicate(stream, recv, mutex_clone, login_key));
                    }
                }
            }
        }
    }
    pub(crate) fn get_input_buffer(&self) -> Arc<Mutex<PlayerInputBuffer>> {
        self.input_stream.clone()
    }
}

impl Deref for PlayerInputBuffer {
    type Target = HashMap<String, VecDeque<Input>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for PlayerInputBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

fn put_buffer(input_buffer: &mut InputBufferMutex, player: String, input: Input) {
    let mut lock = input_buffer.lock().unwrap();
    lock.push_input(player, input);
    drop(lock);
}

fn get_new_view(receiver: &mut Receiver<ClientView>) -> Option<ClientView> {
    match receiver.try_recv() {
        Ok(V) => Some(V),
        Err(E) => match E {
            Empty => None,
            _ => panic!("View channel was closed!")
        }
    }
}

// TODO: Refactor stream reading into its own function
// A note on something that isn't super intuitive here: we close the stream
// if we get an empty buffer back EMPTY_STREAM_PATIENCE number of times.
// Why? Because this would be unexpected behavior
// from the client and signifies that the socket has closed.
const EMPTY_STREAM: [u8; 512] = [0; 512];
const EMPTY_STREAM_PATIENCE: u32 = 2;
const BUFFER_SIZE: usize = 512;
fn stream_communicate(mut stream: TcpStream, mut view_channel: Receiver<ClientView>, mut input_m: Arc<Mutex<PlayerInputBuffer>>, key: String) {
    println!("Connection made!");
    let mut buffer = BytesMut::with_capacity(BUFFER_SIZE);
    buffer.put(&[0; BUFFER_SIZE][..]);
    let mut patience = EMPTY_STREAM_PATIENCE;
    stream.set_nonblocking(true);
    loop {
        // send new data to the client
        let mut view = get_new_view(&mut view_channel);
        if view.is_some() {
            // Get latest view
            loop {
                let mut tmp = get_new_view(&mut view_channel);
                if tmp.is_some() {
                    view = tmp;
                } else {
                    break;
                }
            }
            // Update the client's view:
            update_client_view(&mut stream, view);
        }
        match stream.read(buffer.as_mut()) {
            Ok(_) => {
            },
            Err(e) => if e.kind() == std::io::ErrorKind::WouldBlock {
                continue; // We'll repeat until the stream reads fully
            } else {
                println!("Closing stream due to networking problem: {}", e);
                return;
            }
        }
        if buffer.as_ref() == &EMPTY_STREAM[..] {
            if patience < 1 {
                println!("Closing connection to client"); // TODO: Make it specific
                return;
            } else {
                patience -= 1;
                continue;
            }
        }
        patience = EMPTY_STREAM_PATIENCE; // This only is reached if the buffer was not empty
        let msg = String::from_utf8_lossy(buffer.as_ref());
        let msg: String = msg.chars().take(find_stream_end_chars(msg.to_string())).collect();
        handle_msg(msg);
    }
}

fn handle_msg(msg: String) {
    // TODO: Impl msg parsing
}

fn update_client_view(stream: &mut TcpStream, view: Option<ClientView>) {
    if view.is_none() {
        return;
    }
    // Serialize view
    let ser_view = serde_json::to_string(&view.unwrap()).unwrap() + "\n";
    loop {
        match stream.write(ser_view.as_bytes()) {
            Ok(_) => break,
            Err(e) => if e.kind() == std::io::ErrorKind::WouldBlock {
                continue;
            } else {
                panic!("Socket failed");
            }
        }
    }
    stream.flush();
}