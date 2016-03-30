use serialize::json::Json;

pub struct Bot<'a> {
    awake: bool,
    username: &'a str,
    icon: &'a str,
    channel: Vec<&'a str>,
}

impl<'a> Bot<'a> {
    pub fn new(data: Json) -> Bot<'a> {
        Bot {
            awake: true,
            username: "carl_winslow",
            icon: "",
            channel: vec![""],
        }
    }
}
