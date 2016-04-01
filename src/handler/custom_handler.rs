use api::*;
use websocket::{Message};

pub fn send(message: &Message, command: &str) -> Option<String> {
    let date = regex!(r"^\d{4}-\d{2}-\d{2}$");

    match command {
        "hi" => chat_post_message::send(message, "Hi, it's Carl!"),
        _ if date.is_match(command) => chat_post_message::send(message, "That's a date!"),
        _ => None,
    }
}
