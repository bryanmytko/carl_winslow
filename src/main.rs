#![feature(plugin)]
#![plugin(dotenv_macros)]
extern crate dotenv;
extern crate hyper;
extern crate websocket;
extern crate rustc_serialize as serialize;

use hyper::Client;
use hyper::header::{Headers, ContentType};

use websocket::client::request::Url;
use websocket::{Client as WSClient, Message, Sender, Receiver};
use websocket::message::Type;

use serialize::json::Json;

mod connection;
use connection::Connection;

fn main() {

    let ws_uri = Connection::handshake();
    println!("[Debug] ws_uri: {}", ws_uri);

    let request = WSClient::connect(ws_uri).unwrap();
    let response = request.send().unwrap();

    match response.validate() {
        Ok(()) => { println!("Connected...") },
        Err(e) => { println!("{:?}", e); }
    }

    /* @TODO Spawn threads for send / receive loops */

}
