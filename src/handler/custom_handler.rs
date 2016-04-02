use rtm::*;
use websocket::{Message};

pub fn send(message: &Message, command: &str) -> Option<String> {
    let date = regex!(r"^\d{4}-\d{2}-\d{2}$");

    match command {
        "hi" => message::send(message, "Hi, it's Carl!"),
        _ if date.is_match(command) => message::send(message, "That's a date!"),
        _ => None,
    }
}
