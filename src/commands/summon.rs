
// lib
    // serenity
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::interactions::application_command::ApplicationCommandInteraction;

// crate
use crate::core::command::Command;

use crate::data::repository::banner_content_repository::{
    BannerContentRepository,
    BannerContentRepositoryTrait,
};

use crate::data::repository::unique_character_repository::{
    UniqueCharacterRepository,
    UniqueCharacterRepositoryTrait,
};

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
        
        let banner_content_repository = BannerContentRepository::new(&database);
        let character = banner_content_repository
            .draw_character_from_banner_id(1)
            .await?;

        let unique_character_repository = UniqueCharacterRepository::new(&database);
        let player = Utils::convert_user_id_to_player_model(command.user.id)?;

        unique_character_repository
            .add(&character, &player)
            .await?;

        let result = command.edit_original_interaction_response(
            &context.http, 
            |message| {
                message.content("");
                message.set_embed(character.summon_display(&context.cache))
            }
        ).await;

        if let Err(error) = result {
            return Err(Error::CommandRun(format!("{}", error)));
        }
        
        Ok(())
    }

}
