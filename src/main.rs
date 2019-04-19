extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serenity;

use serenity::{
    client::Client,
    client::Context,
    framework::StandardFramework,
    model::gateway::Ready,
    prelude::EventHandler,
};

mod utils;
mod commands;

struct ClientHandler;

impl EventHandler for ClientHandler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }
}

fn main() {
    env_logger::init();
    let config = utils::core_utils::get_config();
    let mut client = Client::new(&config.discord_bot_token.as_str(), ClientHandler).expect("Error creating client");

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("!").allow_whitespace(true))
            .on("ud", commands::urban_dictionary::cmd),
    );

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

command!(ping(_context, message) {
    let _ = message.reply("Pong!");
});