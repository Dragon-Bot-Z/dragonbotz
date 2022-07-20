
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

impl<'a> PlayerResourceRepository<'a> {
    /// Returns an instance of PlayerResourceRepository
    /// 
    /// ## Arguments:
    /// * database - the database
    pub fn new(database: &'a tokio_postgres::Client) -> Self {
        Self {
            database,
        }
    }
}

#[async_trait]
pub trait PlayerResourceRepositoryTrait {

    /// Returns a PlayerResourceModel instance for the discord id
    /// passed as argument
    /// 
    /// ## Arguments:
    /// * discord_id - the player's discord id
    async fn get_player_resource_with_discord_id(self: &Self, discord_id: &i64)
        -> Result<PlayerResourceModel, Error>;
    
    /// Remove the specified amount of basic summon tickets from the player
    /// 
    /// ## Arguments:
    /// * owner - the owner
    /// * amount - the amount to remove
    async fn remove_summon_ticket_basic_to(self: &Self, 
                                           owner: &PlayerModel, 
                                           amount: i64) 
        -> Result<(), Error>;

}

#[async_trait]
impl PlayerResourceRepositoryTrait for PlayerResourceRepository<'_> {

    async fn get_player_resource_with_discord_id(&self, discord_id: &i64)
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

    async fn remove_summon_ticket_basic_to(&self, 
                                           owner: &PlayerModel, 
                                           amount: i64)
        -> Result<(), Error> {

        let result = self.database
            .execute(
                "UPDATE player_resource
                 SET summon_ticket_base = summon_ticket_base - $1::INT8
                 WHERE owner = $2::INT8",
                 &[&amount, owner.discord_id()]
            ).await;

        if let Err(error) = result {
            return Err(
                Error::DatabaseExecuteError(format!("{}", error))
            )
        }

        Ok(())
    }

}
