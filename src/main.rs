use serenity::{async_trait, model::gateway::Ready, prelude::*};
use crate::config::CONFIG;

mod reddit;
mod config;
mod set_cont;

#[tokio::main]
async fn main() {
    let mut client = Client::builder(CONFIG["Token"].as_str().unwrap())
        .event_handler(Handler)
        .application_id(CONFIG["Application_id"].as_u64().unwrap())
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("online");
    }
}
