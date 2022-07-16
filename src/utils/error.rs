
#[derive(Debug)]
pub enum Error {
    CommandRun(String),
    DatabaseConnectionFailed(String),
    DatabaseQueryError(String),
    DatabaseExecuteError(String),
    EnvironmentVariableParseError(String),
    EnvironmentVariableNotFound(String),
    EnvironmentVariableContainsInvalidCharacters(String),
    RandomCharacterChoosing(String),
    Summon(String),
    UserIdConversion(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            Error::CommandRun(error) => error,
            Error::DatabaseConnectionFailed(error) => error,
            Error::DatabaseQueryError(error) => error,
            Error::DatabaseExecuteError(error) => error,
            Error::EnvironmentVariableParseError(error) => error,
            Error::EnvironmentVariableNotFound(error) => error,
            Error::EnvironmentVariableContainsInvalidCharacters(error) => error,
            Error::RandomCharacterChoosing(error) => error,
            Error::Summon(error) => error,
            Error::UserIdConversion(error) => error,
        };

        write!(f, "{}", content)
    }
}
