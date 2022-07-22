
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

    /// Inserts a new player into the table
    /// tickets
    /// 
    /// ## Arguments:
    /// * discord_id - the player's discord id
    async fn add_player(self: &Self, discord_id: &i64) -> Result<(), Error>;

    /// Returns a PlayerResourceModel instance for the discord id
    /// passed as argument
    /// 
    /// ## Arguments:
    /// * discord_id - the player's discord id
    async fn get_player_resource_with_discord_id(self: &Self, discord_id: &i64)
        -> Result<PlayerResourceModel, Error>;
    
    /// Remove the specified amount of base summon tickets from the player
    /// 
    /// ## Arguments:
    /// * owner - the owner
    /// * amount - the amount to remove
    async fn remove_base_summon_ticket_to(self: &Self, 
                                          owner: &PlayerModel, 
                                          amount: i64) 
        -> Result<(), Error>;

    /// Adds the specified amount of base summon tickets to the player
    /// 
    /// ## Arguments:
    /// * owner - the player that will receive the base summon ticket
    /// * amount - the amount of base summon tickets to add
    async fn add_base_summon_ticket_to(self: &Self,
                                       owner: &PlayerModel,
                                       amount: &i64)
        -> Result<(), Error>;

}

#[async_trait]
impl PlayerResourceRepositoryTrait for PlayerResourceRepository<'_> {

    async fn add_player(&self, discord_id: &i64) -> Result<(), Error> {

        if let Err(error) = self.database
            .execute(
                "INSERT INTO player_resource (owner)
                 VALUES ($1::INT8)", 
                 &[discord_id]
            ).await {

            return Err(Error::DatabaseExecuteError(format!("{} while inserting player data to player resource table", error)))
        }

        Ok(())
    }

    async fn get_player_resource_with_discord_id(&self, discord_id: &i64)
        -> Result<PlayerResourceModel, Error> {

        let result = self.database
            .query_one(
                "SELECT owner,
                        base_summon_ticket
                    
                FROM player_resource
                WHERE owner = $1::INT8",
                &[&discord_id]
            ).await;
        
        if let Err(error) = result {
            return Err(
                Error::DatabaseQueryError(format!("{} while fetching player resources", error))
            )
        }

        let row = result.unwrap();
        let owner = PlayerModel::new_partial_with_discord_id(row.get(0));

        Ok(PlayerResourceModel::new(owner, row.get(1)))
    }

    async fn remove_base_summon_ticket_to(&self, 
                                          owner: &PlayerModel, 
                                          amount: i64)
        -> Result<(), Error> {

        let result = self.database
            .execute(
                "UPDATE player_resource
                 SET base_summon_ticket = base_summon_ticket - $1::INT8
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

    async fn add_base_summon_ticket_to(&self, 
                                       owner: &PlayerModel, 
                                       amount: &i64)
        -> Result<(), Error> {

        if let Err(error) = self.database
            .execute(
                "UPDATE player_resource
                SET base_summon_ticket = base_summon_ticket + $1::INT8
                WHERE owner = $2::INT8",
                &[amount, owner.discord_id()]
            ).await {

            return Err(Error::DatabaseExecuteError(format!("{}", error)))        
        }
        
        Ok(())
    }

}
