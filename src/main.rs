#![feature(plugin)]
#![plugin(dotenv_macros)]
extern crate dotenv;
extern crate hyper;

use std::io::Read;

use hyper::Client;
use hyper::header::Connection;
use hyper::header::{Headers, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel};

fn main() {
    println!("{}", &dotenv!("APIKEY"));

    let client = Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::plaintext());

    /* Example Request
       https://slack.com/api/chat.postMessage?
       token=xxxxxx&channel=%23carls-place&text=hi&pretty=1 */

    let request_string = concat!(
        "token=",
        dotenv!("APIKEY"),
        "&username=carl_winslow&channel=%23carls-place&text=hi&pretty=1"
    );

    println!("{:?}", request_string);

    let mut res = client.post("https://slack.com/api/chat.postMessage")
        .body(request_string)
        .headers(headers)
        .send()
        .unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("Response: {}", body);
}
