use std::io::Read;
use std::io::Write;
use hyper::Client;
use hyper::header::{Headers, ContentType};

use websocket::message::Type;
use websocket::client::request::Url;
use websocket::result::WebSocketResult;
use websocket::{Client as WSClient, Message};
use websocket::stream::WebSocketStream;
use websocket::DataFrame;
use websocket::client::response::Response;

use websocket::sender::Sender;
use websocket::receiver::Receiver;

use serialize::json::Json;

pub struct Connection {
    // client: WSClient<DataFrame, Sender<WebSocketStream>, Receiver<WebSocketStream>>,
    pub sender: Sender<WebSocketStream>,
    pub receiver: Receiver<WebSocketStream>,
}

const MSG_WELCOME: &'static str = "Connected! Welcome to Carl Winslow Bot. \
    Enter a command:\n(type \\q to quit)\n ";

const MSG_CONNECT_ERROR: &'static str = "Could not connect to Slack. Check \
    your API credentials.\n";

impl Connection {
    // pub fn new<R: Read, W: Write>() -> WSClient<DataFrame, Sender<WebSocketStream>, Receiver<WebSocketStream>> {
    pub fn new() -> Connection {
        let ws_uri = Connection::handshake();
        let request = WSClient::connect(ws_uri).unwrap();
        let response = request.send().unwrap();

        match response.validate() {
            Ok(()) => {
              println!("{}", MSG_WELCOME);
              Connection::greeting();
            },
            Err(e) => panic!(MSG_CONNECT_ERROR),
        };

        let (sender, receiver) = response.begin().split();
        Connection {
            sender: sender,
            receiver: receiver
        }
    }

    fn handshake() -> Url {
        let client = Client::new();
        let mut headers = Headers::new();
        headers.set(ContentType::form_url_encoded());

        let request_string = concat!("token=", dotenv!("APIKEY"));

        let mut handshake_request =
            client.post("https://slack.com/api/rtm.start")
            .body(request_string)
            .headers(headers)
            .send()
            .unwrap();

        let mut handshake_response = String::new();
        handshake_request.read_to_string(&mut handshake_response).unwrap();

        let json_response = Json::from_str(&handshake_response).unwrap();
        let json_response_object = json_response.as_object().unwrap();
        let ws_url = json_response_object.get("url").unwrap();
        let ws_url_string = ws_url.as_string().unwrap();

        Url::parse(ws_url_string).unwrap()
    }

    fn greeting() {
        let client = Client::new();
        let mut headers = Headers::new();
        headers.set(ContentType::form_url_encoded());

        // @TODO Generalize greeting -- also, pull this data off the bot data.
        // I think it's available during the handshake.
        let request_string = concat!(
            "token=",
            dotenv!("APIKEY"),
            "&channel=D0TABF474", // Set constant?
            "&text=You%20know%20son%2C%20if%20Screwing%20Up%20ever%20became%20an%20Olympic%20event.%20You%20would%20win%20the%20gold.",
            "&username=carl_winslow",
            "&icon_url=https%3A%2F%2Favatars.slack-edge.com%2F2016-03-17%2F27345813169_aa6498c84afb262aa269_original.jpg"
            );

        let mut message_request =
            client.post("https://slack.com/api/chat.postMessage")
            .body(request_string)
            .headers(headers)
            .send()
            .unwrap();
    }
}
