
// lib
    // std
use std::collections::HashMap;

    // serenity
use serenity::client::Client;
use serenity::model::gateway::GatewayIntents;

// mods
mod commands;
mod core;
mod utils;

// crate
use crate::core::bot::Bot;
use crate::core::command::Command;

use crate::utils::utils::Utils;

// commands
use crate::commands::test::TestCommand;



#[tokio::main]
async fn main() {

    let commands: [Box<dyn Command>; 1] = [
        Box::new(TestCommand),
    ];
    let mut commands_map = HashMap::<String, Box<dyn Command>>::new();

    // add commands to the commands map
    for command in commands {
        commands_map.insert(command.short_name(), command);
    }

    // fetch the environment values
    let token: String = match Utils::environment_value_at("DRAGONBOTZ_TOKEN") {
        Ok(token) => token,
        Err(error) => panic!("{}", error),
    };

    let id_test_guild: u64 = match Utils::environment_value_at("DRAGONBOTZ_TEST_GUILD_ID") {
        Ok(id_test_guild) => id_test_guild,
        Err(error) => panic!("{}", error),
    };

    let id_application: u64 = match Utils::environment_value_at("DRAGONBOTZ_APP_ID") {
        Ok(id_application) => id_application,
        Err(error) => panic!("{}", error),
    };

    // create the client
    let bot = Bot::new(commands_map, id_test_guild, id_application);
    let mut client = match Client::builder(token, GatewayIntents::default())
        .event_handler(bot)
        .application_id(id_application)
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

    // start the client
    if let Err(error) = client.start_autosharded().await {
        Utils::error(
            Utils::exception_message(
                "main", 
                format!("Error starting client: {}", error).as_str()
            ).as_str()
        )
    }

}
