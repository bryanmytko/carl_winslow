use std::io::stdin;
use std::io::{self, Write};

pub struct LoopManager {
    threads: u32,
}

impl LoopManager {
    pub fn new() -> LoopManager {
        LoopManager {
            threads: 0
        }
    }

    pub fn main(&self){
        self.print_prompt();

        loop {
            let mut command = String::new();
            stdin().read_line(&mut command).unwrap();
            let formatted_command = command.trim();

            let message = match formatted_command {
                "\\q" => {
                    println!("Disconnecting!");
                    break;
                },
                // "pint" => Message::ping(b"PING".to_vec()),
                _ => {
                    io::stdout().write(b"Unknown Command!\n");
                    self.print_prompt();
                }
            };

    //      match tx.send(message) {
    //         Ok(()) => (),
    //         Err(e) => {
    //             println!("Main Loop: {:?}", e);
    //             break;
    //         }
    //      }
        }
    }

    fn print_prompt(&self){
        io::stdout().flush();
        io::stdout().write(b"> ");
        io::stdout().flush();
    }
}
