#![feature(plugin)]
#![plugin(dotenv_macros)]
extern crate dotenv;
extern crate hyper;
extern crate rustc_serialize as serialize;
extern crate url;
extern crate websocket;

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

// @TODO Move this
fn text_command(message: &Message){
    let payload = from_utf8(&*message.payload)
        .expect("Invalid payload: {}"); // @TODO

    let json_message = Json::from_str(payload)
        .expect("Unable to parse JSON: {}"); // @TODO

    let json_object = json_message.as_object().expect(""); // @TODO

    match json_object.get("text") {
        Some(command) => {
            let command = command.as_string().expect("asdfa"); // @TODO
            match command {
                "hi" => { api::chat_post_message::send("Hi Carl"); },
                _ => ()
            };
        },
        None => ()
    }
}

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
                Err(e) => { println!("asdf"); return }
            };

            match message.opcode {
                Type::Text => text_command(&message),
                Type::Ping => println!("Ping"),
                Type::Close => return,
                _ => println!("asdfasdf")
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
                    let _ = tx_1.send(Message::pong(message.payload));
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
