
// lib
    // serenity
use serenity::async_trait;
    // tokio-postgres
use tokio_postgres::Config;
use tokio_postgres::NoTls;

// crate
use crate::utils::utils::Utils;
use crate::utils::error::Error;


pub struct Database;

#[async_trait]
trait DatabaseTrait {

    /// Establishes the connection to the database
    /// 
    /// ## Arguments:
    /// * config - the database configuration
    async fn connect(self: &Self, config: &Config) -> Result<(), Error> {
        if let Err(error) = config.connect(NoTls).await {
            return Err(
                Error::DatabaseConnectionFailed(
                    format!("Failed to connect database: {}", error)
                )
            );
        }

        Ok(())
    }

}

impl DatabaseTrait for Database {}

impl Database {

    /// Returns a default config for database connection
    pub fn default_config() -> Config {
        let mut config = Config::new();

        let name: String = match Utils::environment_value_at("DRAGONBOTZ_DB_NAME") {
            Ok(name) => name,
            Err(error) => panic!("{}", error),
        };

        let host: String = match Utils::environment_value_at("DRAGONBOTZ_DB_HOST") {
            Ok(host) => host,
            Err(error) => panic!("{}", error),
        };

        let port: u16 = match Utils::environment_value_at("DRAGONBOTZ_DB_PORT") {
            Ok(port) => port,
            Err(error) => panic!("{}", error),
        };

        let user: String = match Utils::environment_value_at("DRAGONBOTZ_DB_USER") {
            Ok(user) => user,
            Err(error) => panic!("{}", error),
        };

        let password: String = match Utils::environment_value_at("DRAGONBOTZ_DB_PASS") {
            Ok(password) => password,
            Err(error) => panic!("{}", error),
        };

        config.dbname(name.as_str());
        config.host(host.as_str());
        config.port(port);
        config.user(user.as_str());
        config.password(password);
        
        config
    }

}
