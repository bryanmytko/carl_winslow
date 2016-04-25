use rtm::*;
use websocket::{Message};
use handler::{CustomConfig};

pub fn send(message: &Message, command: &str, config: &CustomConfig) -> Option<String> {
    let date = regex!(r"^\d{4}-\d{2}-\d{2}$");
    let exit = regex!(r"^\s*exit[t\s]*ticket[s]?");

    match command {
        "hi" => message::send(message, &config.greeting),
        "type" => typing::send(message),
        _ if exit.is_match(command) => message::send(message, &config.exit_ticket),
        _ if date.is_match(command) => message::send(message, "That's a date!"),
        _ => None,
    }
}
