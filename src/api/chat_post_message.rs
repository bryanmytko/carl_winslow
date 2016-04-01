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
            Msg { _type: ref p_type, id: ref p_id, channel: ref p_channel, text: ref p_text } =>
                encoder.emit_struct("Msg", 2usize, |enc| -> _ {
                    try!(enc.emit_struct_field("type", 0usize, |enc| p_type.encode(enc)));
                    try!(enc.emit_struct_field("channel", 1usize, |enc| p_channel.encode(enc)));
                    try!(enc.emit_struct_field("text", 1usize, |enc| p_text.encode(enc)));
                    return enc.emit_struct_field("id", 1usize, |enc| -> _ { (* p_id).encode(enc) });
                }),
        }
    }
}


pub fn send(message: &str) {
    const METHOD: &'static str = "chat.postMessage";

    let mut headers = ::api::set_headers();
    let client = ::api::set_client(&mut headers);

    // @TODO This stuff is just for testing.
    // Pull actual data off the bot connection.
    let mut request_string = String::new();

    request_string.push_str("token=");
    request_string.push_str(dotenv!("APIKEY"));
    // request_string.push_str("&channel=D0TABF474");
    request_string.push_str("&text=");
    request_string.push_str(message);
    request_string.push_str("&as_user=true");

    let mut request_uri = String::from(::api::API_URI);
    request_uri.push_str(METHOD);

    let _ = client.post(request_uri.as_str())
        .body(&request_string)
        .headers(headers)
        .send();
}

pub fn rtm_send() -> String {
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
