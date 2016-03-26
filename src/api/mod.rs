pub use hyper::Client;
pub use hyper::header::{Headers, ContentType};

pub mod chatPostMessage;

const API_URI: &'static str = "https://slack.com/api/";
// @TODO FOR TESTING. need to pull this off the request
const BOT_IMG: &'static str = "https%3A%2F%2Favatars.slack-edge.com%2F2016-03-17%2F27345813169_aa6498c84afb262aa269_original.jpg";

fn set_headers() -> ::hyper::header::Headers {
    ::hyper::header::Headers::new()
}

fn set_client(headers: &mut ::hyper::header::Headers) -> ::hyper::Client {
    let client = ::hyper::Client::new();
    &headers.set(::hyper::header::ContentType::form_url_encoded());
    client
}
