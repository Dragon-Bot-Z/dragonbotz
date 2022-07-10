
// lib
    // serenity
use serenity::async_trait;

    // tokio-postgres
use tokio_postgres;

// crate
use crate::data::models::player_model::PlayerModel;
use crate::utils::error::Error;


pub struct PlayerRepository<'a> {
    database: &'a tokio_postgres::Client,
}

impl<'a> PlayerRepository<'a> {

    /// Returns an instance of PlayerRepository
    /// 
    /// ## Arguments:
    /// * database - the database client
    pub fn new(database: &'a tokio_postgres::Client) -> Self {
        Self {
            database,
        }
    }

}

#[async_trait]
pub trait PlayerRepositoryTrait {

    /// Returns a player according to the discord id passed as argument
    /// 
    /// ## Argument:
    /// * discord_id - the player's discord id
    async fn get_player_with_discord_id(self: &Self, discord_id: i64) 
        -> Result<PlayerModel, Error>;

}

#[async_trait]
impl PlayerRepositoryTrait for PlayerRepository<'_> {

    async fn get_player_with_discord_id(&self, discord_id: i64)
        -> Result<PlayerModel, Error> {

            let result = self.database
                .query_one(
                    "SELECT *
                    FROM player
                    WHERE discord_id = $1::BIGINT", 
                    &[&discord_id]
                )
                .await;
            
            if let Err(error) = result {
                return Err(Error::DatabaseQueryError(format!("{}", error)));
            }

            let row = result.unwrap();
            
            Ok(PlayerModel::new(row.get(0), row.get(1)))
    }   

}
