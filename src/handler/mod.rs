use websocket::{Message};
use rustc_serialize::json::{Json};
use std::str::from_utf8;

pub mod custom_handler;

pub fn push(message: &Message) -> Option<String> {
    let json_object = parse_payload(message)
        .as_object()
        .and_then(|obj|
            obj.get("text").map(|r| r.clone())
        );

    match json_object {
        Some(c) => {
            let cmd = c.as_string().unwrap_or("");
            custom_handler::send(message, cmd)
        },
        None => None,
    }
}

fn parse_payload(message: &Message) -> Json {
    let payload = from_utf8(&message.payload)
        .expect("Invalid payload: {}");

    Json::from_str(payload).expect("Unable to parse JSON: {}")
}
