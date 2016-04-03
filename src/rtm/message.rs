use std::str::from_utf8;

use rustc_serialize::Encodable;
use rustc_serialize::Encoder;
use rustc_serialize::json::{self, Json};

use websocket::{Message};

static mut MSG_ID: u32 = 0;

struct Msg<'a> {
    id: u32,
    _type: &'a str,
    channel: &'a str,
    text: &'a str,
}

/* Slack's RTM API requires the JSON field `type` which is a reserved word.
 * Define our own Encodable for Msg which maps struct's _text => text */
impl<'a> Encodable for Msg<'a> {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        match * self {
            Msg { _type: ref p_type,
                  id: ref p_id,
                  channel: ref p_channel,
                  text: ref p_text,
                } => {
                encoder.emit_struct("Msg", 2usize, |enc| -> _ {
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
                })
            }
        }
    }
}

pub fn send<'a>(message: &Message, text: &str) -> Option<String> {
    let payload = from_utf8(&message.payload)
        .expect("Invalid payload: {}");

    let parsed_payload = Json::from_str(payload)
        .expect("Unable to parse JSON: {}");

    let channel = parsed_payload
        .find("channel")
        .unwrap()
        .as_string()
        .unwrap();

    let obj = Msg {
        id: unsafe { MSG_ID },
        _type: "message",
        channel: channel,
        text: text
    };

    unsafe { MSG_ID += 1 };

    let encoded = json::encode(&obj).unwrap();

    Some(encoded)
}

pub fn greeting<'a>(channel: &str, text: &str) -> String {
    let obj = Msg {
        id: unsafe { MSG_ID },
        _type: "message",
        channel: channel,
        text: text
    };

    unsafe { MSG_ID += 1 };

    json::encode(&obj).unwrap()
}
