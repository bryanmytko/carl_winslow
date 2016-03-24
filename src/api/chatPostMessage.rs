pub fn send() {
        let client = ::hyper::Client::new();
        let mut headers = ::hyper::header::Headers::new();
        headers.set(::hyper::header::ContentType::form_url_encoded());

        // @TODO Generalize greeting -- also, pull this data off the bot data.
        // I think it's available during the handshake.
        let request_string = concat!(
            "token=",
            dotenv!("APIKEY"),
            "&channel=D0TABF474", // Set constant?
            "&text=Over%20and%20out.",
            "&username=carl_winslow",
            "&icon_url=https%3A%2F%2Favatars.slack-edge.com%2F2016-03-17%2F27345813169_aa6498c84afb262aa269_original.jpg"
            );

        // Gross.
        let mut message_request =
            client.post("https://slack.com/api/chat.postMessage")
            .body(request_string)
            .headers(headers)
            .send()
            .unwrap();
}
