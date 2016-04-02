use std::io::Read;

use hyper::Client;
use hyper::header::{Headers, ContentType};

use url::{ParseError};

use websocket::client::request::{Url as WSUrl};
use websocket::receiver::Receiver;
use websocket::sender::Sender;
use websocket::stream::WebSocketStream;
use websocket::{Client as WSClient};

use rustc_serialize::json::{Json};

pub struct Connection {
    pub sender: Sender<WebSocketStream>,
    pub receiver: Receiver<WebSocketStream>,
    pub self_data: Json
}

const MSG_ONLINE: &'static str = "Connected! Welcome to Carl Winslow Bot. \
    Enter a command:\n(type \\q to quit)\n ";

// const MSG_WELCOME: &'static str = "Carl Winslow is online. \
//     What can I help you with?";

const ERR_RTM_INVALID: &'static str = "RTM response not validated. Check \
    your API credentials.\n";

const ERR_RTM_CONNECTION: &'static str = "Could not reach Slack RTM API. \
    Check connection.\n";

const ERR_INVALID_JSON_URL: &'static str = "Invalid JSON: key `url` missing.\n";

impl Connection {
    pub fn new() -> Connection {
        let response = Connection::rtm_start();

        let ws_uri = Connection::handshake(&response).expect(ERR_RTM_INVALID);
        let ws_request = WSClient::connect(ws_uri).expect(ERR_RTM_CONNECTION);
        let ws_response = ws_request.send().expect(ERR_RTM_CONNECTION);

        let self_data = Connection::self_data(&response);

        match ws_response.validate() {
            Ok(_) => {
                println!("{}", MSG_ONLINE);
                //chat_post_message::send(MSG_WELCOME);
                // request_string.push_str("&channel=);

            },
            Err(e) => panic!(e)
        };

        let (sender, receiver) = ws_response.begin().split();

        Connection {
            sender: sender,
            receiver: receiver,
            self_data: self_data,
        }
    }

    fn rtm_start() -> Json {
        let client = Client::new();
        let mut headers = Headers::new();
        headers.set(ContentType::form_url_encoded());

        let request_string = concat!("token=", dotenv!("APIKEY"));

        let mut handshake_request =
            client.post("https://slack.com/api/rtm.start")
            .body(request_string)
            .headers(headers)
            .send()
            .expect(ERR_RTM_CONNECTION);

        let mut buffer = String::new();
        match handshake_request.read_to_string(&mut buffer) {
            Ok(_) => (),
            Err(e) => panic!(e),
        };

        Json::from_str(&buffer).expect("Invalid JSON: {}")
    }

    fn handshake(response: &Json) -> Result<WSUrl, ParseError> {
        let response_string = response.as_object().and_then(|obj| {
            obj.get("url").and_then(|json| {
                json.as_string()
            })
        }).expect(ERR_INVALID_JSON_URL);

        WSUrl::parse(response_string)
    }

    /* This is confusing. Self refers to the bot whose API token we are using */
    fn self_data(response: &Json) -> Json {
        let json = response.as_object().and_then(|obj| {
            obj.get("self")
        }).expect(ERR_INVALID_JSON_URL);

        json.clone()
    }
}
