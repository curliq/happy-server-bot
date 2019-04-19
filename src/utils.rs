pub const EMBED_COLOR: u64 = 0xf76ab3;

pub mod core_utils {
    use std::fs::File;
    use std::io::prelude::*;

    #[derive(Deserialize)]
    pub struct Config {
        pub discord_bot_token: String
    }

    /// Get the deserialized config object from `secrets.toml` at project root
    pub fn get_config() -> Config {
        let mut content = String::new();
        let _file = File::open("secrets.toml")
            .unwrap()
            .read_to_string(&mut content)
            .unwrap();
        return toml::from_str(&content.as_str()).expect("Failed to parse secrets.toml");
    }
}

pub mod urban_dictionary_api {
    pub const BASE_API: &str = "http://api.urbandictionary.com/v0/define";

    #[derive(Debug)]
    #[derive(Deserialize)]
    pub struct Item {
        pub definition: String,
        pub permalink: String,
        pub thumbs_up: usize,
        pub sound_urls: Vec<String>,
        pub author: String,
        pub word: String,
        pub defid: usize,
        pub current_vote: String,
        pub written_on: String,
        pub example: String,
        pub thumbs_down: usize,
    }

    #[derive(Debug)]
    #[derive(Deserialize)]
    pub struct Response {
        pub list: Vec<Item>
    }
}