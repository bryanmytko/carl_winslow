use std::io::Read;
use std::io::Write;
use hyper::Client;
use hyper::header::{Headers, ContentType};

use url::{Url, ParseResult, ParseError};

use websocket::message::Type;
use websocket::client::request::{Url as WSUrl};
use websocket::result::WebSocketResult;
use websocket::{Client as WSClient, Message};
use websocket::stream::WebSocketStream;
use websocket::DataFrame;
use websocket::client::response::Response;

use websocket::sender::Sender;
use websocket::receiver::Receiver;

use serialize::json::Json;
use serialize::json::{ParserError};

use std::io::Error;
use std::convert;

use api::chatPostMessage;

pub struct Connection {
    pub sender: Sender<WebSocketStream>,
    pub receiver: Receiver<WebSocketStream>,
}

const MSG_ONLINE: &'static str = "Connected! Welcome to Carl Winslow Bot. \
    Enter a command:\n(type \\q to quit)\n ";

const MSG_WELCOME: &'static str = "Carl Winslow is online. What can I help you with?";

const ERR_CONNECT_ERROR: &'static str = "Could not connect to Slack. Check \
    your API credentials.\n";

const ERR_RTM_INVALID: &'static str = "RTM response not validated. Check \
    your API credentials.\n";

const ERR_RTM_CONNECTION: &'static str = "Could not reach Slack RTM API. \
    Check connection.\n";

const ERR_INVALID_JSON_URL: &'static str = "Invalid JSON: key `url` not found.\n";

// #[derive(Debug)]
// enum CliError {
//     Io(io::Error),
//     Parse(num::ParseIntError),
//     Url(io::Error),
// }
//
// impl fmt::Display for CliError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             CliError::Io(ref err) => write!(f, "IO error: {}", err),
//             CliError::Parse(ref err) => write!(f, "Parse error: {}", err),
//         }
//     }
// }
//
// impl error::Error for CliError {
//     fn description(&self) -> &str {
//         // Both underlying errors already impl `Error`, so we defer to their
//         // implementations.
//         match *self {
//             CliError::Io(ref err) => err.description(),
//             CliError::Parse(ref err) => err.description(),
//         }
//     }
//     fn cause(&self) -> Option<&error::Error> {
//         match *self {
//             CliError::Io(ref err) => Some(err),
//             CliError::Parse(ref err) => Some(err),
//         }
//     }
// }


impl Connection {
    pub fn new() -> Connection {
        let ws_uri = Connection::handshake().expect(ERR_RTM_INVALID);
        let request = WSClient::connect(ws_uri).expect(ERR_RTM_CONNECTION);
        let response = request.send().expect(ERR_RTM_CONNECTION);

        match response.validate() {
            Ok(r) => {
                println!("{}", MSG_WELCOME);
                chatPostMessage::send(MSG_WELCOME);
            },
            Err(e) => panic!(ERR_RTM_INVALID)
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
