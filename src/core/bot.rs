
// lib
    // std
use std::collections::HashMap;

    // serenity
use serenity::async_trait;
use serenity::client::Context;

    // tokio-postgres
use tokio_postgres;

use serenity::model::gateway::Ready;
use serenity::model::interactions::Interaction;
use serenity::model::interactions::application_command::ApplicationCommandInteraction;
use serenity::model::id::GuildId;

use serenity::prelude::EventHandler;

// crate
use crate::core::command::Command;


pub struct Bot {

    commands: HashMap<String, Box<dyn Command>>,
    id_test_guild: u64,
    database: tokio_postgres::Client,

}

impl Bot {

    /// Returns an instance of Bot
    /// 
    /// ## Arguments:
    /// * commands - the commands map
    /// * id_test_guild - the test guild id
    /// * database - the database client
    pub fn new(commands: HashMap<String, Box<dyn Command>>,
               id_test_guild: u64,
               database: tokio_postgres::Client) 
        -> Self {

        Self {
            commands,
            id_test_guild,
            database,
        }

    }
    
}


#[async_trait]
trait BotTrait {

    /// Adds slash commands to the test guild
    async fn add_slash_commands_to_test_guild(self: &Self, context: &Context);

    /// Executes the slash command
    async fn execute_slash_command(self: &Self, 
                                   context: &Context, 
                                   command: &ApplicationCommandInteraction);
                                
    /// Checks if the command exists. If the command doesn't exist, displays
    /// an error message to the channel and returns an empty `Err()`
    async fn check_if_slash_command_exists(self: &Self,
                                           context: &Context,
                                           command: &ApplicationCommandInteraction)
        -> Result<(), ()>;

}

#[async_trait]
impl BotTrait for Bot {

    async fn add_slash_commands_to_test_guild(&self, context: &Context) {
        // fetch the test guild
        let test_guild = GuildId(self.id_test_guild);

        // add commands to the test guild
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
            println!("{}", error)
        }
    }

    async fn execute_slash_command(&self, 
                                   context: &Context,
                                   command: &ApplicationCommandInteraction) {
        // check if the command exists
        if let Err(_) = self.check_if_slash_command_exists(&context, &command).await {
            return;
        }

        // tells that the command had been received
        if let Err(error) = command.create_interaction_response(
            &context.http, 
            |response| {
                response.interaction_response_data(|message| message.content("⌛ Processing your request ..."))
            }   
        ).await {
            println!("{}", error)
        };

        // get the command to run
        let mut failed = false;
        let mut failed_error = String::new();
        let command_to_run = &self.commands[&command.data.name];
        if let Err(error) = command_to_run.run(&context, &command).await {
            failed = true;
            failed_error = error;
        };

        // get the original interaction response to delete it
        // or to edit it if the command had failed
        if let Ok(mut message) = command.get_interaction_response(&context.http).await {
            // edit the original interaction response
            if failed {
                if let Err(error) = message.edit(
                    &context.http,
                    |message_| message_.content(format!("❌ Error while processing your request: {}", failed_error))
                ).await {
                    println!("{}", error)
                }

                return;
            }

            // try to delete the original message
            if let Err(error) = message.delete(&context.http).await {
                println!("{}", error)
            }
        }
    }

    async fn check_if_slash_command_exists(&self, 
                                           context: &Context, 
                                           command: &ApplicationCommandInteraction) 
        -> Result<(),()> {
        
        if !self.commands.contains_key(&command.data.name) {
            // send a message to the channel to tell the user 
            // the command doesn't exist
            if let Err(error) = command.create_interaction_response(
                &context.http, 
                |response| {
                    response.interaction_response_data(
                        |message| message.content("❌ Error: Command not found.")
                    )
                }
            ).await {
                println!("{}", error)
            }

            return Err(());
        }
        
        Ok(())
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

    async fn interaction_create(&self, context: Context, interaction: Interaction) {
        if let Some(command) = &interaction.application_command() {
            self.execute_slash_command(&context, &command).await;
        }
    }

}

