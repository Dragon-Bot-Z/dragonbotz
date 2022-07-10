
// lib
    // std
use std::collections::HashMap;

    // serenity
use serenity::client::Client;
use serenity::model::gateway::GatewayIntents;

// mods
mod commands;
mod core;
mod data;
mod utils;

// crate
use crate::core::bot::Bot;
use crate::core::command::Command;
use crate::core::database::Database;
use crate::core::database::DatabaseTrait;

// test
use crate::data::repository::player_repository::{
    PlayerRepository,
    PlayerRepositoryTrait,
};

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

    // creates a database instance
    let database_config = Database::default_config();
    let database_client = match Database::connect(&database_config).await {
        Ok(client) => client,
        Err(error) => panic!("{}", error),
    };
    println!("Database up and running!");

    // create the client
    let bot = Bot::new(commands_map, id_test_guild, database_client);
    let mut client = match Client::builder(token, GatewayIntents::default())
        .event_handler(bot)
        .await {

        Ok(client) => client,
        Err(error) => panic!("{}", error),

    };

    // start the client
    if let Err(error) = client.start_autosharded().await {
        panic!("{}", error)
    }

}
