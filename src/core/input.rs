use crate::utils::InputMap;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Input {
    Click { x: u32, y: u32 },
    Key(char),
}

use self::Input::*;

pub fn key_pressed_for(input_map: &InputMap, user_key: &String, test_key: char) -> bool {
    if let Some(keys) = input_map.get(user_key) {
        for key in keys.iter() {
            if let Key(key) = key {
                if *key == test_key {
                    return true;
                }
            }
        }
    }
    false
}