use std::io::Read;
use hyper::Client;
use hyper::header::{Headers, ContentType};
use websocket::client::request::Url;

use serialize::json::Json;

pub struct Connection;

impl Connection {
    pub fn handshake() -> Url {
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

    pub fn message() {
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
