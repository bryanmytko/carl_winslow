use websocket::{Message};
use serialize::json::{Json};
use std::str::from_utf8;
use std::collections::{BTreeMap};

pub mod custom_handler;

pub fn push(message: &Message){
    let json_object = parse_payload(message)
        .as_object()
        .unwrap()
        .get("text")
        .map(|r| r.clone());

    match json_object {
        Some(c) => {
            let cmd = c.as_string().unwrap_or("");
            custom_handler::send(cmd);
        },
        None => ()
    }
}

fn parse_payload(message: &Message) -> Json {
    let payload = from_utf8(&message.payload)
        .expect("Invalid payload: {}");

    Json::from_str(payload).expect("Unable to parse JSON: {}")
}
