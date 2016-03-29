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
                Err(e) => { println!("Message: Unknown Error."); return } // @TODO
            };

            match message.opcode {
                Type::Text => handler::push(&message),
                Type::Pong => {
                    sender.send_message(&Message::pong(message.payload));
                    prompt::output("Pong!");
                },
                Type::Close => return,
                _ => println!("Unknown opcode: {:?}", message.opcode)
            }
        }
    });

    let receive_loop = thread::spawn(move || {
        for message in receiver.incoming_messages() {
            let message: Message = match message {
                Ok(message) => message,
                Err(e) => { let _ = tx_1.send(Message::close()); return; }
            };

            match message.opcode {
                Type::Text => {
                    let _ = tx_1.send(message);
                },
                Type::Ping => {
                    prompt::output("Ping!");
                    let _ = { tx_1.send(Message::pong(message.payload)); };
                },
                Type::Close => {
                    let _ = tx_1.send(Message::close());
                    return;
                },
                _ => println!("Unknown opcode for message: {:?}", message),
            }
        }
    });

    prompt::display();

    loop {
        let mut buffer = String::new();
        // Remove this expect
        stdin().read_line(&mut buffer)
            .expect("Could not understand that command.");
        let formatted_command = buffer.trim();

        /* Eventually define server side commands here */
        /* @TODO note \q works but breaking the loop exits w/o the messages */
        /* Extract to admin module */
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
    let _ = send_loop.join();
    let _ = receive_loop.join();
    println!("Exited Successfully.");
}
