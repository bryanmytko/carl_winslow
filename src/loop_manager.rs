use std::io::stdin;
use std::io::{self, Write};
use std::sync::mpsc;
use websocket::{Client as WSClient, Message, Sender, Receiver};

pub struct LoopManager {
    threads: u32,
}

impl LoopManager {
    pub fn new() -> LoopManager {
        LoopManager {
            threads: 0
        }
    }

    pub fn main(&self, tx: mpsc::Sender<Message>){
        self.print_prompt();

        loop {
            let mut command = String::new();
            stdin().read_line(&mut command).unwrap();
            let formatted_command = command.trim();

            let message = match formatted_command {
                "\\q" => {
                    println!("Disconnecting!");
                    tx.send(Message::close());
                    return;
                },
                _ => {
                    self.print_prompt();
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
    }

    fn print_prompt(&self){
        io::stdout().flush();
        io::stdout().write(b"> ");
        io::stdout().flush();
    }
}
