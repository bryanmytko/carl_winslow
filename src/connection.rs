use std::io::Read;
use hyper::Client;
use hyper::header::{Headers, ContentType};
use websocket::client::request::Url;

use serialize::json::Json;

pub struct Connection;

impl Connection {
    pub fn handshake() -> Url {
        let client = Client::new();
        /* Can combine these? */
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
}
