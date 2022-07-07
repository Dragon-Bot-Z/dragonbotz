
// lib
    // serenity
use serenity::client::Client;
use serenity::model::gateway::GatewayIntents;

// mods
mod core;
mod utils;

// crate
use crate::core::bot::Bot;
use crate::utils::utils::Utils;


#[tokio::main]
async fn main() {

    let token: String = match Utils::environment_value_at("DRAGONBOTZ_TOKEN") {
        Ok(token) => token,
        Err(error) => panic!("{}", error),
    };

    let bot = Bot;
    let mut client = match Client::builder(token, GatewayIntents::default())
        .event_handler(bot)
        .await {

        Ok(client) => client,
        Err(error) => panic!(
            "{}",
            Utils::exception_message(
                "main", 
                format!("Error creating client: {}", error).as_str()
            )
        )

    };

    if let Err(error) = client.start().await {
        Utils::error(
            Utils::exception_message(
                "main", 
                format!("Error starting client: {}", error).as_str()
            ).as_str()
        )
    }

}
