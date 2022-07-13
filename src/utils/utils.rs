
// lib
    // std
use std::env;

    // chrono
use chrono;
use chrono::Datelike;

    // serenity
use serenity::cache::Cache;
use serenity::builder::CreateEmbed;
use serenity::model::user::CurrentUser;

// crate
use crate::utils::colors::Colors;
use crate::utils::error::Error;


pub struct Utils;

impl Utils {

    /// Returns the value of the environment at key.
    /// 
    /// ## Arguments:
    /// * key - the environment key
    pub fn environment_value_at<T>(key: &str) -> Result<T, Error>
        where T: std::str::FromStr {
        
        match env::var(key) {
            Ok(value) => {

                // convert the value
                match value.parse::<T>() {
                    Ok(converted) => Ok(converted),
                    Err(_) => Err(
                        Error::EnvironmentVariableParseError(
                            format!("Unable to convert \"{}\" to requested type.", key)
                        )
                    )
                }

            } 
            
            Err(env::VarError::NotPresent) => Err(
                Error::EnvironmentVariableNotFound(
                    format!("Environment variable \"{}\" not found.", key)
                )
            ),

            Err(_) => Err(
                Error::EnvironmentVariableContainsInvalidCharacters(
                    format!("Environment variable \"{}\" contains invalid characters.", key)
                )
            ),
        }
    }

    /// Returns the current year
    pub fn current_year() -> i32 {
        chrono::Utc::now().year()
    }

    /// Returns the Bot user stored in cache
    /// 
    /// ## Arguments:
    /// * cache - the cache
    pub fn get_bot_user_from_cache(cache: &Cache) -> CurrentUser {
        cache.current_user()
    }

    /// Returns the default bot's embed
    /// 
    /// ## Arguments: 
    /// * cache - the cache
    pub fn default_embed(cache: &Cache) -> CreateEmbed {
        let mut embed = CreateEmbed::default();

        let bot = Utils::get_bot_user_from_cache(cache);

        embed.color(Colors::DEFAULT.value());
        
        embed.author(|author| {
            author.name(&bot.name);
            author.icon_url(&bot.face())
        });
        embed.footer(|footer| {
            footer.icon_url(&bot.face());
            footer.text(
                format!(
                    "Dragon Bot Z - Elias & Lahcene Belhadi © 2019 - {} | GNU/GPL License",
                    Utils::current_year()
                )
            )
        });

        embed
    }

}

#[cfg(test)]
mod utils_test {
    use super::*;

    #[test]
    fn should_return_error_on_bad_key() {
        assert!(Utils::environment_value_at::<String>("bad_key").is_err())
    }

    #[test]
    fn should_return_ok_on_good_key() {
        assert!(Utils::environment_value_at::<String>("DRAGONBOTZ_TOKEN").is_ok())
    }

    #[test]
    fn should_return_error_if_unable_to_convert_type() {
        assert!(Utils::environment_value_at::<u64>("DRAGONBOTZ_TOKEN").is_err())
    }

}
