#![feature(plugin)]
#![plugin(dotenv_macros)]
#![plugin(regex_macros)]

extern crate dotenv;
extern crate hyper;
extern crate regex;
extern crate rustc_serialize as serialize;
extern crate url;
extern crate websocket;

use std::io::stdin;
use std::sync::mpsc;
use std::thread;

use websocket::message::Type;
use websocket::{Message, Sender, Receiver};

use connection::Connection;

mod api;
mod connection;
mod handler;
mod prompt;

fn main() {
    let connection = Connection::new();
    let mut sender = connection.sender;
    let mut receiver = connection.receiver;

    let (tx, rx) = mpsc::channel();
    let tx_1 = tx.clone();

    let send_loop = thread::spawn(move || {
        loop {
            let message: Message = match rx.recv() {
                Ok(message) => message,
                Err(e) => { println!("{}", e); continue; }
            };

            match message.opcode {
                Type::Text => handler::push(&message),
                Type::Pong => {
                    sender.send_message(&Message::pong(message.payload));
                    prompt::output("Pong!");
                },
                Type::Close => { break; }
                _ => println!("Unknown opcode: {:?}", message.opcode)
            }
        }
    });

    let receive_loop = thread::spawn(move || {
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
                tx.send(Message::close());
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
