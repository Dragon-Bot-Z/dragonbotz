
// lib
    // tokio-postgres
use tokio_postgres;

    // serenity
use serenity::async_trait;

// crate
use crate::data::models::player_resource_model::PlayerResourceModel;
use crate::data::models::player_model::PlayerModel;

use crate::utils::error::Error;


pub struct PlayerResourceRepository<'a> {
    database: &'a tokio_postgres::Client,
}

#[async_trait]
pub trait PlayerResourceRepositoryTrait {

    /// Returns a PlayerResourceModel instance for the discord id
    /// passed as argument
    /// 
    /// ## Arguments:
    /// * discord_id - the player's discord id
    async fn get_player_resource_with_discord_id(self: &Self, discord_id: i64)
        -> Result<PlayerResourceModel, Error>;

}

#[async_trait]
impl PlayerResourceRepositoryTrait for PlayerResourceRepository<'_> {

    async fn get_player_resource_with_discord_id(&self, discord_id: i64)
        -> Result<PlayerResourceModel, Error> {

        let result = self.database
            .query_one(
                "SELECT owner,
                        summon_ticket_base
                    
                FROM player_resource
                WHERE owner = $1::INT8",
                &[&discord_id]
            ).await;
        
        if let Err(error) = result {
            return Err(
                Error::DatabaseQueryError(format!("{}", error))
            )
        }

        let row = result.unwrap();
        let owner = PlayerModel::new_partial_with_discord_id(row.get(0));
        
        Ok(PlayerResourceModel::new(owner, row.get(1)))
    }

}
