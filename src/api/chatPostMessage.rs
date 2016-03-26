pub fn send(message: &str) {
    const METHOD: &'static str = "chat.postMessage";

    let mut headers = ::api::set_headers();
    let client = ::api::set_client(&mut headers);

    // @TODO This stuff is just for testing. Pull actual data off the bot connection.
    let mut request_string = String::new();
    request_string.push_str("token=");
    request_string.push_str(dotenv!("APIKEY"));
    request_string.push_str("&channel=D0TABF474");
    request_string.push_str("&text=");
    request_string.push_str(message);
    request_string.push_str("&username=carl_winslow");
    request_string.push_str("&icon_url=");
    request_string.push_str(::api::BOT_IMG);

    let mut request_uri = String::from(::api::API_URI);
    request_uri.push_str(METHOD);

    let mut message_request =
        client.post(request_uri.as_str())
        .body(&request_string)
        .headers(headers)
        .send()
        .unwrap();
}
