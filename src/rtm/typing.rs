use std::str::from_utf8;

use rustc_serialize::Encodable;
use rustc_serialize::Encoder;
use rustc_serialize::json::{self, Json};

use websocket::{Message};

static mut TYPING_ID: u32 = 0;

struct Typing<'a> {
    id: u32,
    _type: &'a str,
    channel: &'a str,
}

/* Slack's RTM API requires the JSON field `type` which is a reserved word. */
impl<'a> Encodable for Typing<'a> {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        match * self {
            Typing { _type: ref p_type,
                  id: ref p_id,
                  channel: ref p_channel,
                } => {
                encoder.emit_struct("Typing", 2usize, |enc| -> _ {
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

pub fn send<'a>(message: &Message) -> Option<String> {
    let payload = from_utf8(&message.payload)
        .expect("Invalid payload: {}");

    let parsed_payload = Json::from_str(payload)
        .expect("Unable to parse JSON: {}");

    let channel = parsed_payload
        .find("channel")
        .unwrap()
        .as_string()
        .unwrap();

    let obj = Typing {
        id: unsafe { TYPING_ID },
        _type: "typing",
        channel: channel
    };

    unsafe { TYPING_ID += 1 };

    let encoded = json::encode(&obj).unwrap();

    Some(encoded)
}
