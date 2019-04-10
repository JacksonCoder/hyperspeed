use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::{JoinHandle, spawn};
use super::world::Input;
use std::collections::{HashMap, VecDeque};
use bytes::{BytesMut, BufMut};
use std::ops::{Deref, DerefMut};

type InputBufferMutex = Arc<Mutex<PlayerInputBuffer>>;

struct Message {

}

pub(crate) struct PlayerInputBuffer {
    inner: HashMap<String, VecDeque<Input>>
}


pub(crate) struct Server {
    tcp_listener: TcpListener,
    input_stream: InputBufferMutex,
    threads: Vec<JoinHandle<()>>
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
    pub(crate) fn new(s: ServerConfig) -> Server {
        Server {
            tcp_listener: TcpListener::bind(format!("0.0.0.0:{}", s.port)).unwrap(),
            input_stream: Arc::new(Mutex::new(PlayerInputBuffer::new())),
            threads: vec![]
        }
    }
    pub(crate) fn main_loop(&mut self) {
        for stream in self.tcp_listener.incoming() {
            let stream = stream.unwrap();
            let mutex_clone = self.input_stream.clone();
            let key = "asdf".to_string();
            let j_handle = spawn(move || stream_communicate(stream, mutex_clone, key));
            self.threads.push(j_handle);
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

use std::io::Read;

const BUFFER_SIZE: usize = 512;

fn find_stream_end_chars(msg: String) -> usize {
    let mut sequential_exclamations = 0;
    for character in msg.chars().rev() {
        if character == '!' {
            sequential_exclamations += 1;
        } else {
            sequential_exclamations = 0;
        }
        if sequential_exclamations >= 3 {
            return msg.find(character).unwrap();
        }
    }
    return msg.len() - 1;
}

fn put_buffer(input_buffer: &mut InputBufferMutex, player: String, input: Input) {
    let mut lock = input_buffer.lock().unwrap();
    lock.push_input(player, input);
    drop(lock);
}

fn stream_communicate(mut stream: TcpStream, mut input_m: Arc<Mutex<PlayerInputBuffer>>, key: String) {
    println!("Connection made!");
    let mut buffer = Vec::with_capacity(BUFFER_SIZE);
    buffer.put(&[0; BUFFER_SIZE][..]);
    loop {
        stream.read(buffer.as_mut()).unwrap();
        if buffer.len() > 0 {
            let msg = String::from_utf8_lossy(buffer.as_ref());
            let msg: String = msg.chars().take(find_stream_end_chars(msg.to_string())).collect();
            println!("{}", msg);
            if msg == "space" {
                put_buffer(&mut input_m, key.clone(), Input::Key("Space".to_string()));
            }
        }
    }
}