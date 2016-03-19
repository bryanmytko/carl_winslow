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
mod loop_manager;

use connection::Connection;
use loop_manager::LoopManager;

fn main() {

    let ws_uri = Connection::handshake();
    println!("[Debug] ws_uri: {}", ws_uri);

    let request = WSClient::connect(ws_uri).unwrap();
    let response = request.send().unwrap();

    match response.validate() {
        Ok(()) => {
            println!("\nConnected! Welcome to Carl Winslow Bot. Enter a command:");
            println!("(type \\q to quit)\n");

            let loop_manager = LoopManager::new();
            loop_manager.main();
        },
        Err(e) => { println!("Error {:?}", e); }
    }

// let receive_loop = thread::spawn(move || {
// // Receive loop
// for message in receiver.incoming_messages() {
// let message: Message = match message {
// Ok(m) => m,
// Err(e) => {
// println!("Receive
// Loop:
// {:?}",
// e);
// let
// _ = tx_1.send(Message::close());
// return;
// }
// };
// match message.opcode {
// Type::Close => {
// // Got
// a close message, so send a
// close message and return
// let _ = tx_1.send(Message::close());
// return;
// }
// Type::Ping =>
// match
// tx_1.send(Message::pong(message.payload))
// {
// //
// Send a
// pong
// in
// response
// Ok(())
// =>
// (),
// Err(e)
// =>
// {
// println!("Receive
// Loop:
// {:?}",
// e);
// return;
// }
// },
// //
// Say
// what
// we
// received
// _
// =>
// println!("Receive
// Loop:
// {:?}",
// message),
// }
// }
// });
//
}
