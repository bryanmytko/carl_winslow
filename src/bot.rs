use rustc_serialize::json::Json;

pub struct Bot<'a> {
    awake: bool,
    username: &'a str,
    icon: &'a str,
    channel: Vec<&'a str>,
}

impl<'a> Bot<'a> {
    pub fn new(data: &'a Json) -> Bot<'a> {
        let username = data.as_object().and_then(|obj| {
            obj.get("name").and_then(|json| {
                json.as_string().clone()
            })
        });

        Bot {
            awake: true,
            username: username.unwrap_or("bot"),
            icon: "",
            channel: vec![""],
        }
    }
}
