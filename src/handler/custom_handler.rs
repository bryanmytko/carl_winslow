use api::*;
use regex::Regex;

pub fn send(command: &str) {
    let date = regex!(r"^\d{4}-\d{2}-\d{2}$");

    match command {
        "hi" => { chat_post_message::send("Hi, it's Carl!"); },
        _ if date.is_match(command) => { chat_post_message::send("That's a date!"); },
        _ => (),
    };
}
