
// lib
    // tokio-postgres
use tokio_postgres::Client;

    // serenity
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::interactions::application_command::ApplicationCommandInteraction;

// crate
use crate::utils::error::Error;


#[async_trait]
pub trait Command: Send + Sync {

    /// Returns the command's name
    fn name(self: &Self) -> String;

    /// Returns the command's short name
    fn short_name(self: &Self) -> String;

    /// Returns the command's description
    fn description(self: &Self) -> String;

    /// Executes the command's process
    /// 
    /// ## Arguments:
    /// * context - the command's context
    /// * interaction - the interaction that had triggered the command
    async fn run(self: &Self, 
                 _context: &Context, 
                 _interaction: &ApplicationCommandInteraction,
                 _database: &tokio_postgres::Client)
        -> Result<(), Error>;

}
