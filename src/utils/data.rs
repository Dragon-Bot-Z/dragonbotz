
// lib
    // tokio-postgres
use tokio_postgres;

    // serenity
use serenity::async_trait;

// crate
use crate::data::models::player_model::PlayerModel;
use crate::data::repository::player_repository::{
    PlayerRepository,
    PlayerRepositoryTrait,
};

use crate::data::repository::player_resource_repository::{
    PlayerResourceRepository,
    PlayerResourceRepositoryTrait,
};

use crate::utils::error::Error;


pub struct DataUtils;

#[async_trait]
pub trait DataUtilsTrait {

    /// Inserts the player into all the tables
    /// 
    /// ## Arguments:
    /// * database - the database client
    /// * player - the player to insert
    async fn insert_player_into_all_tables(database: &tokio_postgres::Client, 
                                           player: &PlayerModel) 
        -> Result<(), Error>;

}

#[async_trait]
impl DataUtilsTrait for DataUtils {

    async fn insert_player_into_all_tables(database: &tokio_postgres::Client, 
                                           player: &PlayerModel) 
        -> Result<(), Error> {

        let player_repository = PlayerRepository::new(&database);
        let player_resource_repository = PlayerResourceRepository::new(&database);

        player_repository.add(&player.discord_id()).await?;
        player_resource_repository.add_player(&player.discord_id()).await?;

        Ok(())
    }

}
