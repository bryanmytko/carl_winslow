use std::io::Read;
use hyper::Client;
use hyper::header::{Headers, ContentType};

use url::{ParseError};

use websocket::client::request::{Url as WSUrl};
use websocket::{Client as WSClient};
use websocket::stream::WebSocketStream;

use websocket::sender::Sender;
use websocket::receiver::Receiver;

use serialize::json::Json;

use api::chat_post_message;

pub struct Connection {
    pub sender: Sender<WebSocketStream>,
    pub receiver: Receiver<WebSocketStream>,
}

const MSG_ONLINE: &'static str = "Connected! Welcome to Carl Winslow Bot. \
    Enter a command:\n(type \\q to quit)\n ";

const MSG_WELCOME: &'static str = "Carl Winslow is online. What can I help you with?";

const ERR_RTM_INVALID: &'static str = "RTM response not validated. Check \
    your API credentials.\n";

const ERR_RTM_CONNECTION: &'static str = "Could not reach Slack RTM API. \
    Check connection.\n";

const ERR_INVALID_JSON_URL: &'static str = "Invalid JSON: key `url` not found.\n";

impl Connection {
    pub fn new() -> Connection {
        let ws_uri = Connection::handshake().expect(ERR_RTM_INVALID);
        let request = WSClient::connect(ws_uri).expect(ERR_RTM_CONNECTION);
        let response = request.send().expect(ERR_RTM_CONNECTION);

        match response.validate() {
            Ok(_) => {
                println!("{}", MSG_ONLINE);
                chat_post_message::send(MSG_WELCOME);
            },
            Err(e) => panic!(e)
        };

        let (sender, receiver) = response.begin().split();

        Connection {
            sender: sender,
            receiver: receiver
        }
    }

    // @TODO Pattern for implementing From for JSON errors
    // impl<'a, E: Error + 'a> From<E> for Box<Error + 'a>
    // impl From<std::num::ParseIntError> for ParserError {
    //     fn from(_: std::num::ParseIntError) -> ParserError {
    //         ParserError{message: "Invalid data type".to_string()}
    //     }
    // }

    fn handshake() -> Result<WSUrl, ParseError> {
        let client = Client::new();
        let mut headers = Headers::new();
        headers.set(ContentType::form_url_encoded());

        let request_string = concat!("token=", dotenv!("APIKEY"));

        let mut handshake_request =
            client.post("https://slack.com/api/rtm.start")
            .body(request_string)
            .headers(headers)
            .send()
            .expect(ERR_RTM_CONNECTION);  // @TODO try!

        let mut buffer = String::new();
        handshake_request.read_to_string(&mut buffer).map_err(|e| { e });

        let response = Json::from_str(&buffer).expect("Invalid JSON: {}"); // @TODO try!

        let response_string = response.as_object().and_then(|obj| {
            obj.get("url").and_then(|json| {
                json.as_string()
            })
        }).expect(ERR_INVALID_JSON_URL);

        WSUrl::parse(response_string)
    }
}
