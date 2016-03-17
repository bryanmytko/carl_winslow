#![feature(plugin)]
#![plugin(dotenv_macros)]
extern crate dotenv;

fn main() {
    println!("{}", &dotenv!("APIKEY"));
}
