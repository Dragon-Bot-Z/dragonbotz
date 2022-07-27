
// lib
    // tokio-postgres
use tokio_postgres;

    // serenity
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::interactions::application_command::ApplicationCommandInteraction;

// crate
use crate::data::repository::player_resource_repository::{
    PlayerResourceRepository,
    PlayerResourceRepositoryTrait,
};

use crate::data::models::player_model::PlayerModel;

use crate::utils::utils::Utils;
use crate::utils::error::Error;
use crate::utils::items::Items;


pub struct Reward;

#[async_trait]
pub trait RewardTrait {

    /// Rewards the player with the specified amount of base summon ticket
    /// 
    /// ## Arguments:
    /// * context - the context
    /// * database - the database
    /// * interaction - the interaction command
    /// * player - the player's discord_id
    /// * amount - the amound to send to the player
    async fn reward_base_summon_ticket(context: &Context,
                                       database: &tokio_postgres::Client,
                                       interaction: &ApplicationCommandInteraction,
                                       player: &PlayerModel,
                                       amount: &i64)
        -> Result<(), Error>;

}

#[async_trait]
impl RewardTrait for Reward {

    async fn reward_base_summon_ticket(context: &Context,
                                       database: &tokio_postgres::Client,
                                       interaction: &ApplicationCommandInteraction,
                                       player: &PlayerModel,
                                       amount: &i64)
        -> Result<(), Error> {

        PlayerResourceRepository::new(&database)
            .add_base_summon_ticket_to(&player, &amount)
            .await?;

        let mut embed = Utils::default_embed(&context.cache);

        embed.thumbnail(interaction.user.face());
        embed.title(format!("{}'s rewards", interaction.user.name));
        embed.description(format!("{}: *x{}*", Items::BaseSummonTicket, amount));

        let original_message = interaction.get_interaction_response(&context.http).await;
        if let Err(error) = original_message {
            return Err(Error::RewardMessage(format!("{}", error)))
        }
        let original_message = original_message.unwrap();

        // updates the old message
        if let Err(error) = interaction.edit_original_interaction_response(
            &context.http, 
            |message| {
                // restore previous message
                message.content(original_message.content);
                for original_embed in original_message.embeds {
                    message.add_embed(original_embed.into());
                }
                message.add_embed(embed)
            }
        ).await {

            return Err(Error::RewardMessage(format!("{}", error)))
        };

        Ok(())
    }

}
