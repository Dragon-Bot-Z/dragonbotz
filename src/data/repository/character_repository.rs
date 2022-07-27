
// lib
    // serenity
use serenity::async_trait;

    // tokio-postgres
use tokio_postgres;

// crate
use crate::data::models::character_model::CharacterModel;
use crate::utils::error::Error;


pub struct CharacterRepository<'a> {
    database: &'a tokio_postgres::Client,
}

impl<'a> CharacterRepository<'a> {

    /// Returns an instance of CharacterRepository
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
pub trait CharacterRepositoryTrait {

    /// Returns a character according to the character id passed as argument
    /// 
    /// ## Argument:
    /// * id - the character's id
    async fn get_character_with_id(self: &Self, id: i32) 
        -> Result<CharacterModel, Error>;

    /// Returns a vector containing all characters stored in the database
    async fn get_all_characters(self: &Self) -> Result<Vec<CharacterModel>, Error>;

    /// Returns the total number of available characters
    async fn get_total_number_of_characters(self: &Self)
        -> Result<i64, Error>;

}

#[async_trait]
impl CharacterRepositoryTrait for CharacterRepository<'_> {

    async fn get_character_with_id(&self, id: i32) 
        -> Result<CharacterModel, Error> {

        let result = self.database
            .query_one(
                "SELECT character.id,
                        character.name,
                        character.rarity,
                        character.image,
                        character.thumbnail,
                        character.is_origins

                FROM character
                WHERE id = $1::INT4", 
                &[&id]
            ).await;

        if let Err(error) = result {
            return Err(Error::DatabaseQueryError(format!("{} while fetching a the character", error)));
        }

        let row = result.unwrap();

        Ok(
            CharacterModel::new(
                row.get(0), row.get(1), row.get(2), 
                row.get(3), row.get(4), row.get(5),
                None
            )
        )
    }

    async fn get_all_characters(&self) -> Result<Vec<CharacterModel>, Error> {

        let result = self.database
            .query(
                "SELECT character.id,
                        character.name,
                        character.rarity,
                        character.image,
                        character.thumbnail,
                        character.is_origins

                FROM character",
                &[] 
            ).await;

        if let Err(error) = result {
            return Err(Error::DatabaseQueryError(format!("{} while fetching all characters", error)));
        }

        let rows = result.unwrap();
        let mut characters = Vec::<CharacterModel>::new();

        for row in rows {
            let character = CharacterModel::new(
                row.get(0), row.get(1), row.get(2), 
                row.get(3), row.get(4), row.get(5),
                None
            );

            characters.push(character);
        }

        Ok(characters)
    }

    async fn get_total_number_of_characters(&self) -> Result<i64, Error> {

        let result = self.database
            .query_one(
                "SELECT COUNT(id)
                FROM character", 
                &[]
            ).await;
        
        if let Err(error) = result {
            return Err(Error::DatabaseQueryError(format!("{error} while fetching the total number of characters")))
        }

        let count = result.unwrap();

        Ok(count.get("count"))
    }

}
