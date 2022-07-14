
// lib
    // tokio-postgres
use tokio_postgres;

    // serenity
use serenity::async_trait;

// crate
use crate::utils::error::Error;

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

}

#[async_trait]
impl BannerContentRepositoryTrait for BannerContentRepository<'_> {

    async fn get_content_with_banner_id(&self, banner_id: i32) 
        -> Result<BannerContentModel, Error> {

        let result = self.database
            .query(
                "SELECT character.*, banner.*
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
                row.get(0), 
                row.get(1), 
                row.get(2),
                row.get(3),
                row.get(4)
            );

            characters.push(character);

            if !banner_assigned {
                banner_id = row.get(5);
                banner_name = row.get(6);
                banner_assigned = true;
            }
        }

        let banner = BannerModel::new(banner_id, banner_name);

        Ok(BannerContentModel::new(characters, banner))
    }

}
