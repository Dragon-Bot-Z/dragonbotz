
// lib
    // std
use std::collections::HashMap;

    // serenity
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::interactions::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::application_command::CommandDataOption;
use serenity::builder::CreateApplicationCommandOption;

// crate
use crate::utils::error::Error;


#[async_trait]
pub trait Command: Send + Sync {

    /// Returns the command's name
    fn name(self: &Self) -> String;

    /// Returns the command's short name
    fn short_name(self: &Self) -> String;

    /// Returns the command's description
    fn description(self: &Self) -> String;

    /// Tells if the players has to be registered to use this command
    fn player_needs_to_exist(self: &Self) -> bool { true }

    /// Tells if the players has to not exist to use this command
    fn player_has_to_not_exist(self: &Self) -> bool { false }

    /// Tells if the command has options
    fn has_options(self: &Self) -> bool {
        if let Some(options) = self.options() {
            options.len() > 0
        } else {
            false
        }
    }

    /// Returns the command options
    fn options(self: &Self) -> Option<Vec<CreateApplicationCommandOption>> {
        None
    }

    /// Returns a Map of options passed by the user
    /// 
    /// ## Arguments:
    /// * interaction - the command interaction
    fn options_map(self: &Self, interaction: &ApplicationCommandInteraction)
        -> Option<HashMap<String, CommandDataOption>> {

        let options = interaction
            .data
            .options
            .clone();

        if options.len() == 0 {
            return None
        }

        let mut options_map = HashMap::<String, CommandDataOption>::new();
        for option in options {
            options_map.insert(option.name.clone(), option.clone());
        }

        Some(options_map)
    }

    /// Executes the command's process
    /// 
    /// ## Arguments:
    /// * context - the command's context
    /// * interaction - the interaction that had triggered the command
    async fn run(self: &Self, 
                 _context: &Context, 
                 _interaction: &ApplicationCommandInteraction,
                 _database: &tokio_postgres::Client)
        -> Result<(), Error>;

}
