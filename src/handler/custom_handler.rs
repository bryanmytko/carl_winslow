use rtm::*;
use websocket::{Message};
use handler::{CustomConfig};

pub fn send(message: &Message, command: &str, config: &CustomConfig) -> Option<String> {
    let date = regex!(r"^\d{4}-\d{2}-\d{2}$");
    let exit = regex!(r"^\s*exit[t\s]*ticket[s]?");
    let set_exit = regex!(r"^set exit ticket\s+(\S+)");

    match command {
        "hi" => message::send(message, &config.greeting),
        "type" => typing::send(message),
        _ if exit.is_match(command) => message::send(message, &config.exit_ticket),
        _ if date.is_match(command) => message::send(message, "That's a date!"),
        _ if set_exit.is_match(command) => {
            let mut string = config.set_exit_ticket.clone();

            for capture in set_exit.captures_iter(command) {
                // @TODO make a link
                // @TOOO save to db
                let exit = capture.at(1).unwrap_or("");
                string.push_str(exit);
            }

            message::send(message, &string)

        }
        _ => None,
    }
}
