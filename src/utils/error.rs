
#[derive(Debug)]
pub enum Error {
    BoxCommand(String),
    CommandRun(String),
    DatabaseConnectionFailed(String),
    DatabaseQueryError(String),
    DatabaseExecuteError(String),
    EnvironmentVariableParseError(String),
    EnvironmentVariableNotFound(String),
    EnvironmentVariableContainsInvalidCharacters(String),
    InventoryCommand(String),
    NotEnoughResources(String),
    RandomCharacterChoosing(String),
    RewardMessage(String),
    StartCommand(String),
    SummonCommand(String),
    UserIdConversion(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            Error::BoxCommand(error) => format!("Box error: {}", error),
            Error::CommandRun(error) => format!("Command execution error: {}", error),
            Error::DatabaseConnectionFailed(error) => format!("Database connection error: {}", error),
            Error::DatabaseQueryError(error) => format!("Database Query error: {}", error),
            Error::DatabaseExecuteError(error) => format!("Database execute error: {}", error),
            Error::EnvironmentVariableParseError(error) => format!("Command execution: {}", error),
            Error::EnvironmentVariableNotFound(error) => format!("Environment variable not found error: {}", error),
            Error::EnvironmentVariableContainsInvalidCharacters(error) => format!("Environment variable contains invalid characters error: {}", error),
            Error::InventoryCommand(error) => format!("Inventory command error: {}", error),
            Error::NotEnoughResources(error) => format!("Not enough resources error: {}", error),
            Error::RandomCharacterChoosing(error) => format!("Random character choosing error: {}", error),
            Error::StartCommand(error) => format!("Start command error: {}", error),
            Error::RewardMessage(error) => format!("Reward message error: {}", error),
            Error::SummonCommand(error) => format!("Summon command error: {}", error),
            Error::UserIdConversion(error) => format!("User id conversion error: {}", error),
        };

        write!(f, "{}", content)
    }
}
