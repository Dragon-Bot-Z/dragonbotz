
// lib
    // std
use std::env;


pub struct Utils;

impl Utils {

    /// Returns a formatted error message.
    /// 
    /// ## Arguments:
    /// * where - where the exception had been triggered
    /// * message - the exception's message
    pub fn exception_message(r#where: &str, message: &str) -> String {
        format!("Exception raised in {}: {}.", r#where, message).to_string()
    }

    /// Panics and displays an error message
    /// 
    /// ## Arguments:
    /// * message - the message to display
    pub fn error(message: &str) {
        panic!("{}", message)
    }

    /// Returns the value of the environment at key.
    /// 
    /// ## Arguments:
    /// * key - the environment key
    pub fn environment_value_at<T>(key: &str) -> Result<T, String>
        where T: std::str::FromStr {
        
        match env::var(key) {
            Ok(value) => {

                // convert the value
                match value.parse::<T>() {
                    Ok(converted) => Ok(converted),
                    Err(_) => Err(
                        Utils::exception_message(
                            "environment_value_at", 
                            format!("Unable to parse \"{}\" to the requested type.", key).as_str()
                        )
                    ),
                }

            } 
            
            Err(env::VarError::NotPresent) => Err(
                Utils::exception_message(
                    "environment_value_at",
                    format!("\"{}\" not found", key).as_str()
                )
            ),

            Err(_) => Err(
                Utils::exception_message(
                    "environment_value_at",
                    format!("\"{}\" contains invalid characters", key).as_str()
                )
            )

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
