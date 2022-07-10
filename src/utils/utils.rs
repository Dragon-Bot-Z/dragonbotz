
// lib
    // std
use std::env;

// crate
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
