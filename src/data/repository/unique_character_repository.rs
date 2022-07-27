
// lib
    // tokio
use tokio;

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

    /// Returns the players characters and their count
    /// 
    /// ## Arguments:
    /// * owner - the player
    async fn get_player_unique_characters_and_count(self: &Self, owner: &PlayerModel)
        -> Result<Vec<(CharacterModel, i64)>, Error>;

    /// Returns the players unique characters according to the unique id passed
    /// 
    /// ## Arguments:
    /// * owner - the player
    /// * id - the character id
    async fn get_player_unique_characters_with_id(self: &Self, 
                                                  owner: &PlayerModel,
                                                  id: &i64)
        -> Result<Vec<CharacterModel>, Error>;

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

    async fn get_player_unique_characters_and_count(&self, owner: &PlayerModel)
        -> Result<Vec<(CharacterModel, i64)>, Error> {
        
        let result = self.database
            .query(
                "SELECT character.id,
                        character.name,
                        character.rarity,
                        character.image,
                        character.thumbnail,
                        character.is_origins,
                        COUNT(character.id)
                    
                FROM character
                INNER JOIN unique_character
                ON unique_character.character = character.id
                WHERE unique_character.owner = $1::INT8
                GROUP BY character.id
                ORDER BY character.id ASC", 
                &[owner.discord_id()]
            ).await;
        
        if let Err(error) = result {
            return Err(Error::DatabaseQueryError(format!("{} while fetching the player's box", error)))
        }

        let player_characters = result.unwrap();

        let characters_join = tokio::spawn(
            async move {
                let mut characters: Vec<(CharacterModel, i64)> = vec![];
                for row in player_characters {
                    let character = CharacterModel::new(
                        row.get(0),
                        row.get(1),
                        row.get(2),
                        row.get(3),
                        row.get(4),
                        row.get(5),
                        None
                    );

                    // add the Character and the count
                    characters.push((character, row.get(6)));
                }
                
                characters
            }
        );

        let characters = match characters_join.await {
            Ok(characters) => characters,
            Err(error) => return Err(Error::BoxCommand(format!("{} while asynchronously fetching player's box", error)))
        };

        Ok(characters)
    }

    async fn get_player_unique_characters_with_id(&self, 
                                                  owner: &PlayerModel, 
                                                  id: &i64)
        -> Result<Vec<CharacterModel>, Error> {

        let result = self.database
            .query(
                "SELECT unique_character.id AS unique_id,
                        unique_character.character,
                        character.id,
                        character.name,
                        character.rarity,
                        character.image,
                        character.thumbnail,
                        character.is_origins
                
                FROM unique_character
                INNER JOIN character
                ON character.id = unique_character.character
                WHERE owner = $1::INT8 AND character.id = $2::INT8", 
                &[owner.discord_id(), id]
            ).await;

        if let Err(error) = result {
            return Err(Error::DatabaseQueryError(format!("{error} while fetching player's unique character with id")))
        }
        let rows = result.unwrap();

        let mut characters = Vec::<CharacterModel>::new();
        for row in rows {
            let character = CharacterModel::new(
                row.get("id"), 
                row.get("name"), 
                row.get("rarity"), 
                row.get("image"), 
                row.get("thumbnail"), 
                row.get("is_origins"), 
                Some(row.get("unique_id"))
            );

            characters.push(character);
        }

        Ok(characters)
    }

}

