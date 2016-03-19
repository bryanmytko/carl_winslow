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

use serialize::json::Json;
use std::thread;
use std::sync::mpsc;
use std::str::from_utf8;

mod connection;
mod loop_manager;

use connection::Connection;
use loop_manager::LoopManager;

const MSG_WELCOME: &'static str = "\nConnected! Welcome to Carl Winslow Bot. \
    Enter a command:\n(type \\q to quit)\n ";

fn main() {
    let ws_uri = Connection::handshake();
    println!("[Debug] ws_uri: {}", ws_uri);

    let request = WSClient::connect(ws_uri).unwrap();
    let response = request.send().unwrap();

    response.validate().unwrap();

    match response.validate() {
        Ok(()) => {
          println!("{}", MSG_WELCOME);
        },
        Err(e) => { println!("Error {:?}", e); }
    }

    // @TODO Channels should be extracted from main

    let (mut sender, mut receiver) = response.begin().split();
    let (tx, rx) = mpsc::channel();
    let tx_1 = tx.clone();

    let send_loop = thread::spawn(move || {
        loop {
            let message = match rx.recv() {
                Ok(message) => {
                    println!("Message: {:?}", message);
                },
                Err(e) => {
                    println!("Send Loop: {:?}", e);
                    return;
                },
            };
            // match message.opcode {
            //     Type::Close => {
            //         let _ =
            //             sender.send_message(&message);
            //             return;
            //     },
            //     _ => (),
            // }
            // // Send the message
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

                    // @TODO parse from String(&str)
                    match msg_object.get("text") {
                        Some(s) => println!("Slack Message: {:?}", &s),
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

    // @TODO Child threads need to exit
    println!("Waiting for child threads to exit");

    let _ = send_loop.join();
    let _ = receive_loop.join();

    println!("Exited");
}
