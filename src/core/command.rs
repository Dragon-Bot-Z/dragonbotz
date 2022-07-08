
// lib
    // serenity
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::interactions::Interaction;


#[async_trait]
pub trait Command {

    async fn run(_context: &Context, _interaction: &Interaction);

}
