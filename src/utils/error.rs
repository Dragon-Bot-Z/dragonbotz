
pub enum Error {
    DatabaseConnectionFailed(String),
    EnvironmentVariableParseError(String),
    EnvironmentVariableNotFound(String),
    EnvironmentVariableContainesInvalidCharacters(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            Error::DatabaseConnectionFailed(error) => error,
            Error::EnvironmentVariableParseError(error) => error,
            Error::EnvironmentVariableNotFound(error) => error,
            Error::EnvironmentVariableContainesInvalidCharacters(error) => error,
        };

        write!(f, "{}", content)
    }
}
