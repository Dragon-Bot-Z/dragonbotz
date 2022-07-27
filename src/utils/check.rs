
// lib
    // tokio-postgres
use tokio_postgres;

    // serenity
use serenity::async_trait;

// crate
use crate::utils::error::Error;


pub struct Check;

#[async_trait]
pub trait CheckTrait {

    /// Checks if a user exists in the database
    /// 
    /// ## Arguments:
    /// * database - the database
    /// * discord_id - the use discord id
    async fn user_exists(database: &tokio_postgres::Client, discord_id: &i64)
        -> Result<bool, Error>;

}

#[async_trait]
impl CheckTrait for Check {

    async fn user_exists(database: &tokio_postgres::Client, discord_id: &i64) 
        -> Result<bool, Error> {

        let result = database
            .query_one(
                "SELECT COUNT(1)
                 FROM player
                 WHERE discord_id = $1::INT8", 
                &[discord_id]
            ).await;

        if let Err(error) = result {
            return Err(
                Error::DatabaseQueryError(format!("{} while checking if the user exists", error))
            )
        }

        let row = result.unwrap();
        if row.get::<usize, i64>(0) == 0 {
            return Ok(false);
        }

        Ok(true)
    }

}
