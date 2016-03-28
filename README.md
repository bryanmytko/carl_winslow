# Carl Winslow Bot

![Carl Winslow](assets/img.jpg)

### Creating a bot

Create a bot on [Slack's website](https://api.slack.com/bot-users) and add the Api key to a .env file:

    `APIKEY=xxxxxxxxxxxxxxxxxxxxx`

### Custom handlers

You can create custom handlers for string and pattern matches in:
**/src/handler/custom_handler.rs**

    let p = regex!(r"^\d{4}-\d{2}-\d{2}$");

    match command {
        _ if p.is_match(command) => { chat_post_message::send("Some response text here"); },
        _ => (),
    };

### Exiting

`\q` command will exit the program and take the bot offline. While this command currently works, it's not as clean as it could be as the threads crash instead of silently joining.

### Connecting

Currently only supports nightly

    rustc 1.9.0-nightly (b12b4e4e3 2016-03-17)
    cargo 0.10.0-nightly (ece4e96 2016-03-17)

Compile and run:

`cargo build`

` ./target/debug/carl_winslow_bot`

or

`cargo run`

The bot will receive a `ping` via RTM API from Slack every ~30 seconds and replies to keep the bot connected. This `ping` / `pong` exchange currently displays in the console (but should eventually have a --quiet mode to hide this output).
