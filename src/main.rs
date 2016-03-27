#![feature(plugin)]
#![plugin(dotenv_macros)]
extern crate dotenv;
extern crate hyper;
extern crate websocket;
extern crate rustc_serialize as serialize;
extern crate url;

use serialize::json::{Json};

use std::io::stdin;
use std::str::from_utf8;
use std::sync::mpsc;
use std::thread;

use websocket::message::Type;
use websocket::{Message, Receiver};

use connection::Connection;

mod api;
mod connection;
mod prompt;

fn main() {
    let connection = Connection::new();
    let mut sender = connection.sender;
    let mut receiver = connection.receiver;

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

            // @TODO match opcode for disconnect
            // match message.opcode {
            //     Type::Close => sender.send_message(&Message::close()),
            //     _ => (),
            // };
        }
    });

    /* Receives messages via WS */
    let receive_loop = thread::spawn(move || {
        for message in receiver.incoming_messages() {
            let message: Message = match message {
                Ok(m) => { m },
                Err(e) => {
                    println!("Receive Loop: {:?}", e);
                    let _ = tx_1.send(Message::close());
                    return;
                }
            };

            match message.opcode {
                Type::Text => {
                    let payload = from_utf8(&*message.payload)
                        .expect("Invalid payload: {}");

                    let message = Json::from_str(payload)
                        .expect("Unable to parse JSON: {}");

                    let parsed_message = message.as_object().and_then(|obj| {
                        match obj.get("text") {
                            /* @TODO Extract this entire mess */
                            Some(v) => {
                                let v = v.as_string();
                                match v.unwrap() { // @TODO fix, gross
                                    "hi" => { api::chat_post_message::send("Oh, Hi!"); },
                                    _ => ()
                                };
                                return v;
                            },
                            None => Some("Text opcode with no text value.")
                        }
                    }).unwrap();

                    println!("Slack message: {}", parsed_message);
                },
                Type::Close => {
                    let _ = tx_1.send(Message::close());
                    return;
                },
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

    prompt::display();

    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer)
            .expect("Could not understand that command.");
        let formatted_command = buffer.trim();

        /* Eventually define server side commands here */
        /* @TODO note \q works but breaking the loop exits w/o the messages */
        let message = match formatted_command {
            "\\q" => {
                println!("Disconnecting!");
                tx.send(Message::close());
                return;
            },
            _ => {
                prompt::display();
                Message::text(formatted_command.to_owned())
            }
        };

        match tx.send(message) {
            Ok(()) => (),
            Err(e) => {
                println!("Main Loop: {:?}", e); // debug
                break;
            }
        }
    }

    println!("Waiting for child threads to exit...");
    send_loop.join();
    receive_loop.join();
    println!("Exited Successfully.");
}
