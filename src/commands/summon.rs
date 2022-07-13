
// lib
    // tokio-postgres
use tokio_postgres::Client;

    // serenity
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::interactions::application_command::ApplicationCommandInteraction;

    // rand
use rand::seq::SliceRandom;

// crate
use crate::core::command::Command;

use crate::data::repository::character_repository::*;

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
                 command: &ApplicationCommandInteraction,
                 database: &tokio_postgres::Client)
        -> Result<(), Error> {

        // get random character from the database
        let character_repo = CharacterRepository::new(&database);
        let characters_result = character_repo.get_all_characters().await;

        if let Err(error) = characters_result {
            return Err(Error::CommandRun(format!("{}", error)));
        }

        let characters = characters_result.unwrap();
        let character_option = characters.choose(&mut rand::thread_rng());

        if let None = character_option {
            return Err(Error::Summon("Character is None.".to_string()));
        }

        let character = character_option.unwrap();

        let mut embed = Utils::default_embed(&context.cache);
        embed.description(
            format!(
                "Name: **{}** - `#{}`
Rarity: {}",
                character.name(),
                character.id(),
                character.rarity_converted()
            )
        );

        embed.thumbnail(character.thumbnail());
        embed.image(character.image());

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
