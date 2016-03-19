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

use std::io::stdin;
use std::io::{self, Write};

mod connection;
use connection::Connection;

fn main() {

    let ws_uri = Connection::handshake();
    println!("[Debug] ws_uri: {}", ws_uri);

    let request = WSClient::connect(ws_uri).unwrap();
    let response = request.send().unwrap();

    match response.validate() {
        Ok(()) => {
            println!("\nConnected! Welcome to Carl Winslow Bot. Enter a command:");
            println!("(type \\q to quit)\n");
            print_prompt();
        },
        Err(e) => { println!("Error {:?}", e); }
    }

    /* Start main loop */
    loop {
        let mut command = String::new();
        stdin().read_line(&mut command).unwrap();
        let formatted_command = command.trim();


        let message = match formatted_command {
            "\\q" => {
                println!("Disconnecting!");
                break;
            },
            _ => {
                io::stdout().write(b"Unknown Command!\n");
                print_prompt();
            }
            // "ping" => {
            //     Message::ping(b"PING".to_vec()),
            //     _ => Message::text(trimmed.to_string()),
            //  }
        };

        // match tx.send(message) {
        //     Ok(()) => (),
        //     Err(e) => {
        //         println!("Main Loop: {:?}", e);
        //         break;
        //     }
        // }
    }

    fn print_prompt(){
        io::stdout().flush();
        io::stdout().write(b"> ");
        io::stdout().flush();
    }
}
