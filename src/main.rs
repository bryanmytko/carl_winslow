#![feature(plugin)]
#![plugin(dotenv_macros)]
extern crate dotenv;
extern crate hyper;
extern crate websocket;
extern crate rustc_serialize as serialize;

use std::io::Read;

use hyper::Client;
use hyper::header::{Headers, ContentType};

use websocket::client::request::Url;
use websocket::{Client as WSClient, Message, Sender, Receiver};
use websocket::message::Type;

use serialize::json::Json;

fn main() {
    /* @TODO Move to handshake / connect module */
    let client = Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::form_url_encoded());

    let request_string = concat!(
        "token=",
        dotenv!("APIKEY")
    );

    let mut handshake =
        client.post("https://slack.com/api/rtm.start")
        .body(request_string)
        .headers(headers)
        .send()
        .unwrap();

    let mut response = String::new();
    handshake.read_to_string(&mut response).unwrap();

    let json_response = Json::from_str(&response).unwrap();
    let json_response_object = json_response.as_object().unwrap();
    let ws_url = json_response_object.get("url").unwrap();
    let ws_url_string = ws_url.as_string().unwrap();

    let uri = Url::parse(ws_url_string).unwrap();

    println!("{}", uri);

    let request = WSClient::connect(uri).unwrap();
    let response = request.send().unwrap();

    match response.validate() {
        Ok(()) => { println!("Connected...") },
        Err(e) => { println!("{:?}", e); }
    }

    /* @TODO Spawn threads for send / receive loops */

}
