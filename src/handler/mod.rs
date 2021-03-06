use rustc_serialize::json::{Json};
use std::fs::File;
use std::io::prelude::*;
use std::str::from_utf8;
use toml;
use toml::{Value};
use websocket::{Message};

pub mod custom_handler;

#[derive(RustcDecodable)]
pub struct CustomConfig {
    pub greeting: String,
    pub exit_ticket: String,
    pub set_exit_ticket: String,
}

pub struct Handler {
    config: CustomConfig
}

impl Handler {
    pub fn new() -> Handler {
        Handler {
            config: Handler::load_custom_config()
        }
    }

    fn load_custom_config() -> CustomConfig {
        /* @TODO implement better error handling */
        /* @TODO if no file, build default Config struct */
        println!("[DEBUG] I'm about to load a file");
        let mut file = File::open("Custom.toml").unwrap();
        let mut toml_data = String::new();
        file.read_to_string(&mut toml_data).unwrap();

        let mut parser = toml::Parser::new(&toml_data);
        let toml = parser.parse();
        let config = Value::Table(toml.unwrap());

        let config: CustomConfig = match toml::decode(config) {
            Some(t) => t,
            None => panic!("Could not parse Custom.toml! Aborting.")
        };

        config
    }

    pub fn push(&self, message: &Message) -> Option<String> {
        let json_object = self.parse_payload(message)
            .as_object()
            .and_then(|obj|
                obj.get("text").map(|r| r.clone())
            );

        match json_object {
            Some(c) => {
                let cmd = c.as_string().unwrap_or("");
                custom_handler::send(message, cmd, &self.config)
            },
            None => None,
        }
    }

    fn parse_payload(&self, message: &Message) -> Json {
        let payload = from_utf8(&message.payload)
            .expect("Invalid payload: {}");

        Json::from_str(payload).expect("Unable to parse JSON: {}")
    }
}
