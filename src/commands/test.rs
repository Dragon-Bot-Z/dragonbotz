
// lib
    // tokio-postgres
use tokio_postgres::Client;

    // serenity
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::interactions::application_command::ApplicationCommandInteraction;

// crate
use crate::core::command::Command;
use crate::utils::error::Error;


pub struct TestCommand;


#[async_trait]
impl Command for TestCommand {

    fn name(self: &Self) -> String {
        "Test".to_string()
    }

    fn short_name(self: &Self) -> String {
        "test".to_string()
    }

    fn description(self: &Self) -> String {
        "A simple test command".to_string()
    }

    async fn run(&self, 
                 context: &Context, 
                 command: &ApplicationCommandInteraction,
                 _: &tokio_postgres::Client) 
        -> Result<(), Error> { 

        let channel = command.channel_id;

        if let Err(error) = channel.send_message(&context.http, |message| message.content("Hey!")).await {
            return Err(Error::CommandRun(format!("{}", error)));
        }

        Ok(())
    }
    
}
