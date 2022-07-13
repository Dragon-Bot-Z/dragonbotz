
// lib
    // serenity
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::interactions::application_command::ApplicationCommandInteraction;

// crate
use crate::core::command::Command;

use crate::utils::utils::Utils;
use crate::utils::error::Error;


pub struct SummonCommand;

#[async_trait]
impl Command for SummonCommand {
    
    fn name(&self) -> String {
        "Summon".to_string()
    }

    fn short_name(&self) -> String {
        "summon".to_string()
    }

    fn description(&self) -> String {
        "Allows you to summon characters".to_string()
    }

    async fn run(&self, 
                 context: &Context, 
                 command: &ApplicationCommandInteraction)
        -> Result<(), Error> {

        let mut embed = Utils::default_embed(&context.cache);

        let channel = command.channel_id;
        let result = channel
            .send_message(&context.http, |message| message.set_embed(embed))
            .await;

        if let Err(error) = result {
            return Err(Error::CommandRun(format!("{}", error)));
        }
        
        Ok(())
    }

}
