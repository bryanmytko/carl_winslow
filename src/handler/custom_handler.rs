use std::fs::File;
use std::io::prelude::*;
use rtm::*;
use toml;


use websocket::{Message};

pub fn send(message: &Message, command: &str) -> Option<String> {
    let date = regex!(r"^\d{4}-\d{2}-\d{2}$");

    let mut file = File::open("Handler.toml").unwrap();
    let mut toml = String::new();

    file.read_to_string(&mut toml).unwrap();
    let toml = toml::Parser::new(&toml).parse().unwrap();
    let response_data = toml.get("response").unwrap();
    let response_data = response_data.as_table().unwrap();
    let welcome_text = response_data.get("welcome").unwrap().as_str().unwrap();

    match command {
        "hi" => message::send(message, welcome_text),
        "type" => typing::send(message),
        _ if date.is_match(command) => message::send(message, "That's a date!"),
        _ => None,
    }
}
