
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

use crate::utils::reward::{
    Reward,
    RewardTrait,
};

use crate::utils::data::*;


pub struct StartCommand;

#[async_trait]
impl Command for StartCommand {

    fn name(&self) -> String {
        "Start".to_string()
    }

    fn short_name(&self) -> String {
        "start".to_string()
    }

    fn description(&self) -> String {
        "Allows you to start your Dragon Bot Z journey".to_string()
    }

    fn player_needs_to_exist(&self) -> bool { false }

    fn player_has_to_not_exist(&self) -> bool { true }

    async fn run(&self, 
                 context: &Context, 
                 interaction: &ApplicationCommandInteraction, 
                 database: &tokio_postgres::Client)
        -> Result<(), Error> {
        
        let player = Utils::convert_user_id_to_player_model(&interaction.user.id)?;
        
        DataUtils::insert_player_into_all_tables(&database, &player).await?;

        let mut embed = Utils::default_embed(&context.cache);

        embed.title(
            format!(
                "{}, your Dragon Bot Z journey begins ... now !", interaction.user.name
            )
        );

        embed.description(
            format!(
                "Hello {} and welcome to Dragon Bot Z ! We do really hope your journey will be fun and entertaining ! 
Please accept this gift of {}, it should help you start your journey",
                interaction.user.name,
                Items::BaseSummonTicket
            ),
        );

        let edit_result = interaction.edit_original_interaction_response(
            &context.http, 
            |message| {
                message.content("");
                message.set_embed(embed)
            }
        ).await;

        if let Err(error) = edit_result {
            return Err(Error::StartCommand(format!("{}", error)))
        }

        let base_summon_tickets = 5;
        Reward::reward_base_summon_ticket(&context, &database, &interaction, &player, &base_summon_tickets).await?;

        Ok(())
    }

}
