
#[derive(Debug)]
pub enum Error {
    CommandRun(String),
    DatabaseConnectionFailed(String),
    DatabaseQueryError(String),
    DatabaseExecuteError(String),
    EnvironmentVariableParseError(String),
    EnvironmentVariableNotFound(String),
    EnvironmentVariableContainsInvalidCharacters(String),
    NotEnoughResources(String),
    RandomCharacterChoosing(String),
    Summon(String),
    UserIdConversion(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            Error::CommandRun(error) => format!("Command execution error: {}", error),
            Error::DatabaseConnectionFailed(error) => format!("Database connection error: {}", error),
            Error::DatabaseQueryError(error) => format!("Database Query error: {}", error),
            Error::DatabaseExecuteError(error) => format!("Database execute error: {}", error),
            Error::EnvironmentVariableParseError(error) => format!("Command execution: {}", error),
            Error::EnvironmentVariableNotFound(error) => format!("Environment variable not found error: {}", error),
            Error::EnvironmentVariableContainsInvalidCharacters(error) => format!("Environment variable contains invalid characters error: {}", error),
            Error::NotEnoughResources(error) => format!("Player not enough resources error: {}", error),
            Error::RandomCharacterChoosing(error) => format!("Random character choosing error: {}", error),
            Error::Summon(error) => format!("Summon error: {}", error),
            Error::UserIdConversion(error) => format!("User id conversion error: {}", error),
        };

        write!(f, "{}", content)
    }
}
