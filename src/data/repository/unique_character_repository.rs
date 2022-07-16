
// lib
    // tokio-postgres
use tokio_postgres;

    // serenity
use serenity::async_trait;

// crate
use crate::utils::error::Error;

use crate::data::models::character_model::CharacterModel;
use crate::data::models::player_model::PlayerModel;


pub struct UniqueCharacterRepository<'a> {
    database: &'a tokio_postgres::Client,
}

impl<'a> UniqueCharacterRepository<'a> {

    /// Returns an instance of UniqueCharacterRepository
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
pub trait UniqueCharacterRepositoryTrait {

    /// Adds a character to the unique character table
    /// 
    /// ## Arguments:
    /// * character - the character to add
    /// * owner - the owner of the character
    async fn add(self: &Self, character: &CharacterModel, owner: &PlayerModel)
        -> Result<(), Error>;

}

#[async_trait]
impl UniqueCharacterRepositoryTrait for UniqueCharacterRepository<'_> {

    async fn add(&self, character: &CharacterModel, owner: &PlayerModel)
        -> Result<(), Error> {

        let execute_result = self.database
            .execute(
                "INSERT INTO unique_character
                VALUES(DEFAULT, $1::INT4, $2::INT8)",
                &[&character.id(), &owner.discord_id()]
            ).await;
        
        match execute_result {
            Ok(_) => Ok(()),
            Err(error) => Err(
                Error::DatabaseExecuteError(format!("{}", error))
            )
        }
    }

}

