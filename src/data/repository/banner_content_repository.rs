
// lib
    // tokio-postgres
use tokio_postgres;

    // rand
use rand::seq::SliceRandom;

    // serenity
use serenity::async_trait;

// crate
use crate::utils::error::Error;
use crate::utils::utils::Utils;
use crate::utils::droprate::DropRate;
use crate::utils::rarity::Rarity;

use crate::data::models::banner_model::BannerModel;
use crate::data::models::banner_content_model::BannerContentModel;
use crate::data::models::character_model::CharacterModel;


pub struct BannerContentRepository<'a> {
    database: &'a tokio_postgres::Client,
}

impl<'a> BannerContentRepository<'a> {

    /// Returns an instance of BannerContentRepository
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
pub trait BannerContentRepositoryTrait {

    /// Returns banner content according to the banner id passed
    /// as parameter
    /// 
    /// ## Argument:
    /// * banner_id - the banner id
    async fn get_content_with_banner_id(self: &Self, banner_id: i32)
        -> Result<BannerContentModel, Error>;

    /// Returns the banner characters with the corresponding rarity
    ///
    /// ## Arguments:
    /// * banner_id - the banner id
    /// * rarity - the rarity
    async fn get_content_with_banner_id_rarity(self: &Self, 
                                               banner_id: i32,
                                               rarity: Rarity)
        -> Result<Vec<CharacterModel>, Error>;

    /// Returns the banner origins characters
    ///
    /// ## Arguments:
    /// * banner_id - the banner id
    async fn get_content_origins_with_banner_id(self: &Self, banner_id: i32)
        -> Result<Vec<CharacterModel>, Error>;

    /// Returns a random character from the banner according to 
    /// the rarity droprates
    /// 
    /// ## Arguments:
    /// * banner_id - the banner from which to draw the character
    async fn draw_character_from_banner_id(self: &Self, banner_id: i32)
        -> Result<CharacterModel, Error>;

}

#[async_trait]
impl BannerContentRepositoryTrait for BannerContentRepository<'_> {

    async fn get_content_with_banner_id(&self, banner_id: i32) 
        -> Result<BannerContentModel, Error> {

        let result = self.database
            .query(
                "SELECT character.id,
                        character.name,
                        character.rarity,
                        character.image,
                        character.thumbnail,
                        character.is_origins,
                        banner.id,
                        banner.name
                FROM banner
                INNER JOIN banner_content 
                ON (banner_content.banner = banner.id)
                INNER JOIN character 
                ON (banner_content.character = character.id)
                WHERE banner_content.banner = $1::INT4
                ",
                &[&banner_id]
            ).await;
        
        if let Err(error) = result {
            return Err(Error::DatabaseQueryError(format!("{}", error)))
        }

        let mut characters = Vec::<CharacterModel>::new();
        let mut banner_id: i32 = 0;
        let mut banner_name = String::new();
        let mut banner_assigned = false;

        let rows = result.unwrap();
        for row in rows {
            let character = CharacterModel::new(
                row.get(0), row.get(1), row.get(2), 
                row.get(3), row.get(4), row.get(5)
            );

            characters.push(character);

            if !banner_assigned {
                banner_id = row.get(6);
                banner_name = row.get(7);
                banner_assigned = true;
            }
        }

        let banner = BannerModel::new(banner_id, banner_name);

        Ok(BannerContentModel::new(characters, banner))
    }

    async fn get_content_with_banner_id_rarity(&self, 
                                               banner_id: i32, 
                                               rarity: Rarity)
        -> Result<Vec<CharacterModel>, Error> {

        let result = self.database
            .query(
                "SELECT character.id,
                        character.name,
                        character.rarity,
                        character.image,
                        character.thumbnail,
                        character.is_origins

                FROM banner
                INNER JOIN banner_content 
                ON (banner_content.banner = banner.id)
                INNER JOIN character
                ON (banner_content.character = character.id)
                WHERE banner_content.banner = $1::INT4
                AND character.rarity = $2::INT2
                AND character.is_origins = false", 
                &[&banner_id, &rarity.id()]
            ).await;
        
        if let Err(error) = result {
            return Err(Error::DatabaseQueryError(format!("{}", error)))
        }

        let mut characters = Vec::<CharacterModel>::new();
        let rows = result.unwrap();

        for row in rows {
            let character = CharacterModel::new(
                row.get(0), row.get(1), row.get(2), 
                row.get(3), row.get(4), row.get(5)
            );

            characters.push(character);
        }

        Ok(characters)
    }

    async fn get_content_origins_with_banner_id(&self, banner_id: i32)
        -> Result<Vec<CharacterModel>, Error> {

        let result = self.database
            .query(
                "SELECT character.id,
                        character.name,
                        character.rarity,
                        character.image,
                        character.thumbnail,
                        character.is_origins
                
                FROM banner
                INNER JOIN banner_content
                ON (banner_content.banner = banner.id)
                INNER JOIN character
                ON (banner_content.character = character.id)
                WHERE banner_content.banner = $1::INT4
                AND character.is_origins = true",
                &[&banner_id]
            ).await;
        
        if let Err(error) = result {
            return Err(Error::DatabaseQueryError(format!("{}", error)))
        }

        let rows = result.unwrap();
        let mut characters = Vec::<CharacterModel>::new();

        for row in rows {
            let character = CharacterModel::new(
                row.get(0), row.get(1), row.get(2), 
                row.get(3), row.get(4), row.get(5)
            );

            characters.push(character);
        }

        Ok(characters)
    }

    async fn draw_character_from_banner_id(&self, banner_id: i32)
        -> Result<CharacterModel, Error> {

        let draw = Utils::random_float();
        println!("Draw: {}", draw);
        let mut characters: Vec<CharacterModel> = vec![];

        if draw <= DropRate::ORIGINS.value() {
            characters = self
                .get_content_origins_with_banner_id(banner_id)
                .await?
                .clone();

        } else if draw <= DropRate::EXTREME.value() {
            characters = self
                .get_content_with_banner_id_rarity(banner_id, Rarity::EXTREME)
                .await?
                .clone();
            
        } else if draw <= DropRate::SUPER.value() {
            characters = self
                .get_content_with_banner_id_rarity(banner_id, Rarity::SUPER)
                .await?
                .clone();

        } else if draw <= DropRate::UNCOMMON.value() {
            characters = self
                .get_content_with_banner_id_rarity(banner_id, Rarity::UNCOMMON)
                .await?
                .clone();

        } else {
            characters = self
                .get_content_with_banner_id_rarity(banner_id, Rarity::COMMON)
                .await?
                .clone();
        }

        let character_option = characters.choose(&mut rand::thread_rng());

        if let None = character_option {
            return Err(
                Error::RandomCharacterChoosing("Empty character".to_string())
            );
        }

        let dropped_character = character_option.unwrap();

        Ok(dropped_character.clone())
    }

}
