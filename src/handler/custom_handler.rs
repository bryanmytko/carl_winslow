use rtm::*;
use websocket::{Message};

pub fn send(message: &Message, command: &str) -> Option<String> {
    let date = regex!(r"^\d{4}-\d{2}-\d{2}$");
    /* @TODO Implement a way to save the exit ticket URL */
    let exit_ticket = regex!(r"^\s*exi\s*[t\s]+icket");

    match command {
        "hi" => message::send(message, "Hi, it's Carl!"),
        "type" => typing::send(message),
        _ if date.is_match(command) => message::send(message, "That's a date!"),
        _ if exit_ticket.is_match(command) => message::send(message, "[Exit Ticket]"),
        _ => None,
    }
}
