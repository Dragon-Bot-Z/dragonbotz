
// lib
    // serenity
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::gateway::Ready;
use serenity::prelude::EventHandler;


pub struct Bot;

#[async_trait]
impl EventHandler for Bot {

    async fn ready(&self, _: Context, _: Ready) {
        println!("Connected!");
    }

}

