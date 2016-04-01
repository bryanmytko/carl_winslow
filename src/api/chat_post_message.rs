use rustc_serialize::Encodable;
use rustc_serialize::Encoder;
use rustc_serialize::json::{self, ToJson, Json};
use std::str::from_utf8;
use websocket::{Message};

struct Msg<'a> {
    id: u32,
    _type: &'a str,
    channel: &'a str,
    text: &'a str,
}

impl<'a> Encodable for Msg<'a> {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        match * self {
            Msg {
                _type: ref p_type,
                id: ref p_id,
                channel: ref p_channel,
                text: ref p_text
            } => encoder.emit_struct("Msg", 2usize, |enc| -> _ {
                    try!(enc.emit_struct_field(
                            "type",
                            0usize,
                            |enc| p_type.encode(enc)
                            )
                    );
                    try!(enc.emit_struct_field(
                            "channel",
                            1usize,
                            |enc| p_channel.encode(enc)
                            )
                    );
                    try!(enc.emit_struct_field(
                            "text",
                            1usize,
                            |enc| p_text.encode(enc)
                            )
                    );
                    enc.emit_struct_field(
                        "id",
                        1usize,
                        |enc| -> _ { (* p_id).encode(enc) }
                    )
                }
            ),
        }
    }
}


pub fn send<'a>(message: &Message, text: &str) -> String {
    let obj = Msg {
        id: 12343,
        _type: "message",
        channel: "D0TABF474",
        text: "Hello world"
    };

    let encoded = json::encode(&obj).unwrap(); //.to_string();

    //let msg_r = sender.send_message(Message::text(encoded));

    // let payload = from_utf8(&msg_r.payload).expect("Invalid payload: {}");
    // let payload_str = Json::from_str(payload).expect("Unable to parse JSON: {}");

    // Message::text(encoded)
    encoded
}

pub fn rtm_send(message: &str) -> String {
    let obj = Msg {
        id: 12343,
        _type: "message",
        channel: "D0TABF474",
        text: "Hello world"
    };

    let encoded = json::encode(&obj).unwrap(); //.to_string();

    //let msg_r = sender.send_message(Message::text(encoded));

    // let payload = from_utf8(&msg_r.payload).expect("Invalid payload: {}");
    // let payload_str = Json::from_str(payload).expect("Unable to parse JSON: {}");

    // Message::text(encoded)
    encoded
}
