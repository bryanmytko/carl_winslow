use api::*;
use regex::Regex;

pub fn send(command: &str) {
    let hello = regex!(r"[:digit]");
    // assert!(re.is_match("2014-01-01"));

    match command {
        "hi" => { chat_post_message::send("Hi, it's Carl!"); },
        hello => { chat_post_message::send("Not much."); },
        // _ => ()
    };
}
