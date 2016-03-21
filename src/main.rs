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

use std::io::stdin;
use std::io::{self, Write};
use std::collections::BTreeMap;

use serialize::json::{self, Json, ToJson};
use std::thread;
use std::sync::mpsc;
use std::str::from_utf8;

mod connection;
mod loop_manager;

use connection::Connection;
use loop_manager::LoopManager;

const MSG_WELCOME: &'static str = "\nConnected! Welcome to Carl Winslow Bot. \
    Enter a command:\n(type \\q to quit)\n ";

struct Msg {
    Id: u32,
    Type: String,
    Channel: String,
    Text: String,
}

impl ToJson for Msg {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("id".to_string(), self.Id.to_json());
        d.insert("type".to_string(), self.Type.to_json());
        d.insert("channel".to_string(), self.Channel.to_json());
        d.insert("text".to_string(), self.Text.to_json());
        Json::Object(d)
    }
}


fn main() {
    let ws_uri = Connection::handshake();
    println!("[Debug] ws_uri: {}", ws_uri);

    let request = WSClient::connect(ws_uri).unwrap();
    let response = request.send().unwrap();

    response.validate().unwrap();

    match response.validate() {
        Ok(()) => {
          println!("{}", MSG_WELCOME);
          Connection::greeting(); // @TODO API placeholder
        },
        Err(e) => { println!("Error {:?}", e); }
    }

    // @TODO Bring threads back to main; looper is not necessary.

    let (mut sender, mut receiver) = response.begin().split();
    let (tx, rx) = mpsc::channel();
    let tx_1 = tx.clone();

    let send_loop = thread::spawn(move || {
        loop {
            let message = match rx.recv() {
                Ok(message) => {
                    println!("Send Loop receives message: {:?}", message);
                },
                Err(e) => {
                    println!("Send Loop Err: {:?}", e);
                    return;
                },
            };

            // @TODO need to match on opcode to close connection
            // match message.opcode {
            //     Type::Close => {
            //         let _ =
            //             sender.send_message(&message);
            //             return;
            //     },
            //     _ => (),
            // }

            // match sender.send_message(&message) {
            //     Ok(()) => (),
            //     Err(e) => {
            //         println!("Send Loop: {:?}", e);
            //         let _ = sender.send_message(&Message::close());
            //         return;
            //     }
            // }
        }
    });


    let receive_loop = thread::spawn(move || {
        for message in receiver.incoming_messages() {
            let message: Message = match message {
                Ok(m) => { println!("Message received: {:?}", m); m },
                Err(e) => {
                    println!("Receive Loop: {:?}", e);
                    let _ = tx_1.send(Message::close());
                    return;
                }
            };
            match message.opcode {
                Type::Text => {
                    let tmp = from_utf8(&*message.payload).unwrap();
                    let msg_json = Json::from_str(tmp).unwrap();
                    let msg_object = msg_json.as_object().unwrap();

                    match msg_object.get("text") {
                        Some(m) => {
                            match m.as_string() {
                                Some(s) => {
                                    // @TODO Responses should get sent to send thread
                                    println!("Slack Message: {:?}", s);
                                }
                                None => println!("[Debug] Text Message: None"),
                            }
                        },
                        // @TODO Do we even care about None here?
                        None => println!("Non-text message"),
                    }
                },
                Type::Close => {
                    let _ = tx_1.send(Message::close());
                    return;
                }
                Type::Ping => match tx_1.send(Message::pong(message.payload)) {
                    Ok(()) => (),
                    Err(e) => {
                        println!("Receive Loop: {:?}", e);
                        return;
                    }
                },
              _ => println!("Receive Loop: {:?}", message),
            }
        }
    });

    let loop_manager = LoopManager::new();
    loop_manager.main(tx);

    println!("Waiting for child threads to exit");

    // @TODO Child threads not actually exiting? Quit command hangs.
    send_loop.join();
    receive_loop.join();

    println!("Exited");
}
