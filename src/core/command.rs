
// lib
    // serenity
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::interactions::Interaction;


#[async_trait]
pub trait Command {

    /// Executes the command's process
    /// 
    /// ## Arguments:
    /// * context - the command's context
    /// * interaction - the interaction that had triggered the command
    async fn run(_context: &Context, _interaction: &Interaction);

}
