
// lib
    // tokio-postgres
use tokio_postgres;

    // serenity
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::interactions::application_command::ApplicationCommandInteraction;

// crate
use crate::core::command::Command;

use crate::utils::error::Error;
use crate::utils::utils::Utils;
use crate::utils::items::Items;

use crate::data::repository::player_resource_repository::{
    PlayerResourceRepository,
    PlayerResourceRepositoryTrait,
};


pub struct InventoryCommand;

#[async_trait]
impl Command for InventoryCommand {

    fn name(&self) -> String {
        "Inventory".to_string()
    }

    fn short_name(&self) -> String {
        "inventory".to_string()
    }

    fn description(&self) -> String {
        "Allows you to consult the items you own".to_string()
    }

    async fn run(&self,
                 context: &Context,
                 interaction: &ApplicationCommandInteraction,
                 database: &tokio_postgres::Client)
        -> Result<(), Error> {
        
        let user = &interaction.user;
        let player = Utils::convert_user_id_to_player_model(&user.id)?;
        let mut embed = Utils::default_embed(&context.cache);

        let player_resource_repository = PlayerResourceRepository::new(&database);
        let player_resources = player_resource_repository
            .get_player_resource_with_discord_id(player.discord_id())
            .await?;
        
        embed.title(format!("{}'s inventory", user.name));
        embed.thumbnail(user.face());
        embed.description(
            format!(
                "{}: *x{}*", 
                Items::BaseSummonTicket,
                player_resources.summon_ticket_base()
            )
        );

        let response_result = interaction.edit_original_interaction_response(
            &context.http, 
            |message| {
                message.content("");
                message.set_embed(embed)
            }
        ).await;

        if let Err(error) = response_result {
            return Err(Error::InventoryCommand(format!("{}", error)));
        }

        Ok(())
    }

}
