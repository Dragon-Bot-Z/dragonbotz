
// lib
    // tokio-postgres
use tokio_postgres;

    // serenity
use serenity::async_trait;

// crate
use crate::utils::error::Error;
use crate::data::models::banner_model::BannerModel;


pub struct BannerRepository<'a> {
    database: &'a tokio_postgres::Client,
}

impl<'a> BannerRepository<'a> {
    
    /// Returns an instance of BannerRepository
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
pub trait BannerRepositoryTrait {

    /// Returns a banner according to the id passed as parameter
    /// 
    /// ## Arguments:
    /// * id - the banner id
    async fn get_banner_with_id(self: &Self, id: i32) 
        -> Result<BannerModel, Error>;

}

#[async_trait]
impl BannerRepositoryTrait for BannerRepository<'_> {

    async fn get_banner_with_id(&self, id: i32)
        -> Result<BannerModel, Error> {

        let result = self.database
            .query_one(
                "SELECT * 
                FROM banner
                WHERE id = $1::INT4", 
                &[&id]
            ).await;
        
        if let Err(error) = result {
            return Err(Error::DatabaseQueryError(format!("{}", error)));
        }

        let row = result.unwrap();

        Ok(BannerModel::new(row.get(0), row.get(1)))
    }

}
