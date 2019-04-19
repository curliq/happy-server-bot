extern crate inflector;
extern crate reqwest;

use inflector::Inflector;
use serenity::{
    builder::CreateEmbed,
    client::Context,
    framework::standard::Args,
    framework::standard::CommandError,
    model::channel::Message,
    utils::Colour,
};

use crate::utils;
use crate::utils::urban_dictionary_api;

pub fn cmd(_context: &mut Context, message: &Message, args: Args) -> Result<(), CommandError> {

    let response: urban_dictionary_api::Response = reqwest::Client::new().get(urban_dictionary_api::BASE_API)
        .query(&[("term", &args.full())])
        .send()?
        .json().unwrap();

    let item = &response.list[0];

    let _ = message.channel_id.send_message(|m| m.embed(|e| CreateEmbed::default()
        .title(item.word.to_title_case())
        .color(Colour::from(utils::EMBED_COLOR))
        .description('\u{200B}') // zero width space
        .field("Definition", &item.definition, false)
        .field("Example", &item.example, false)
        .field("\u{200B}", format!("{} \u{1F44D}, {} \u{1F44E}", &item.thumbs_up, &item.thumbs_down), false)
    )).unwrap();

    Ok(())
}
