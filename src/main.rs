#![feature(plugin)]
#![plugin(dotenv_macros)]
#![plugin(regex_macros)]

extern crate dotenv;
extern crate hyper;
extern crate regex;
extern crate rustc_serialize;
extern crate url;
extern crate websocket;

use std::io::stdin;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use websocket::message::Type;
use websocket::{Message, Sender, Receiver};

use connection::Connection;


use rtm::*;

mod rtm;
mod connection;
mod handler;
mod prompt;

const MSG_WELCOME: &'static str = "Carl Winslow is online. \
    What can I help you with?";

fn main() {
    let connection = Connection::new();
    let mut sender = connection.sender;
    let mut receiver = connection.receiver;

    /* @TODO REMOVE Testing typing/pause/text effect */
    let t = typing::send(&Message::text("")).unwrap();
    let g = message::send(&Message::text(""), MSG_WELCOME).unwrap();
    sender.send_message(&Message::text(t));
    thread::sleep(Duration::from_millis(2000));
    sender.send_message(&Message::text(g));
    /****************************************/

    let (tx, rx) = mpsc::channel();
    let tx_1 = tx.clone();

    /* Send Loop */
    thread::spawn(move || {
        loop {
            let message: Message = match rx.recv() {
                Ok(message) => message,
                Err(e) => { println!("{}", e); continue; }
            };

            match message.opcode {
                Type::Text => {
                    prompt::flush();
                    match handler::push(&message) {
                        Some(p) => {
                            // Explore the possibility of passing a reference to the sender?
                            let _ = sender.send_message(&Message::text(p));
                        },
                        None => println!("Text message with no payload."),
                    };
                },
                Type::Pong => {
                    match sender.send_message(&Message::pong(message.payload)) {
                        Ok(_) => prompt::output("Pong!"),
                        Err(e) => println!("Ping/Pong failed: {}", e),
                    }
                },
                Type::Close => { break; }
                _ => println!("Unknown opcode: {:?}", message.opcode)
            }
        }
    });

    /* Receive Loop */
    thread::spawn(move || {
        for message in receiver.incoming_messages() {
            let message: Message = match message {
                Ok(message) => message,
                Err(e) => { println!("Receiver error: {}", e); continue; }
            };

            match message.opcode {
                Type::Text => {
                    let _ = tx_1.send(message);
                },
                Type::Ping => {
                    prompt::output("Ping!");
                    let _ = tx_1.send(Message::pong(message.payload));
                },
                Type::Close => { let _ = tx_1.send(Message::close()); },
                _ => println!("Unknown opcode for message: {:?}", message),
            }
        }
    });

    prompt::display();

    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        let formatted_command = buffer.trim();

        /* @TODO Extract to admin module. Add commands. */
        match formatted_command {
            "\\q" => {
                println!("Shutting down!");
                let _ = tx.send(Message::close());
                break;
            },
            _ => {
                prompt::display();
                Message::text(formatted_command.to_owned())
            }
        };
    }

    println!("Exited Successfully.");
}
