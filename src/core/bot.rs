
// lib
    // std
use std::collections::HashMap;

    // serenity
use serenity::async_trait;
use serenity::client::Context;

use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;

use serenity::prelude::EventHandler;

// crate
use crate::core::command::Command;
use crate::utils::utils::Utils;


pub struct Bot {

    commands: HashMap<String, Box<dyn Command>>,
    id_test_guild: u64,
    id_application: u64,

}

impl Bot {

    /// Returns an instance of Bot
    /// 
    /// ## Arguments:
    /// * commands - the commands map
    /// * id_test_guild - the test guild id
    /// * id_application - the application id
    pub fn new(commands: HashMap<String, Box<dyn Command>>,
               id_test_guild: u64,
               id_application: u64) 
        -> Self {

        Self {
            commands,
            id_test_guild,
            id_application
        }

    }

}


#[async_trait]
trait BotUtils {

    /// Adds slash commands to the test guild
    async fn add_slash_commands_to_test_guild(self: &Self, context: &Context);

}

#[async_trait]
impl BotUtils for Bot {

    async fn add_slash_commands_to_test_guild(&self, context: &Context) {
        // fetch the test guild
        let test_guild = GuildId(self.id_test_guild);

        // add the commands to the test guild
        if let Err(error) = test_guild.set_application_commands(
            &context.http,
            |application_commands| {

                for (name, command) in &self.commands {
                    application_commands.create_application_command(
                        |new_command| {

                            new_command
                                .name(name)
                                .description(command.description());
                            
                            new_command

                        }
                    );
                }

                application_commands

            }

        ).await {
            println!("{}",
                Utils::exception_message(
                    "Bot::add_slash_commands_to_test_guild", 
                    format!("Unable to create slash command for test guild: {}", error).as_str()
                )
            )
        }
    }

}

#[async_trait]
impl EventHandler for Bot {

    async fn ready(&self, context: Context, _: Ready) {
        println!("Connected!");
        println!("Adding slash commands to the test guild ...");
        // add commands to the test guild
        self.add_slash_commands_to_test_guild(&context).await;
        println!("DONE!");
    }

}

