use std::thread::sleep;

use chrono::{Duration, Utc};
use serenity::{async_trait, model::gateway::Ready, prelude::*};
use serenity::model::id::ChannelId;

use crate::config::CONFIG;
use crate::reddit::get_reddit_value;
use crate::wait::*;

mod config;
mod reddit;
mod set_cont;
mod wait;

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
    async fn ready(&self, ctx: Context, _ready: Ready) {
        println!("online");
        loop {
            wait_till_next_saturday();
            let mut time = Utc::now();
            let data = get_reddit_value();
            for i in 0..CONFIG["Per_day"].as_u64().unwrap() as usize {
                let item = &data.get(i).unwrap()["data"];
                if let Err(why) = ChannelId(CONFIG["Channel_id"].as_u64().unwrap())
                    .send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            if !item["is_video"].as_bool().unwrap()
                                || item["media"]["reddit_video"]["is_gif"].as_bool().unwrap()
                            {
                                e.image(item["url"].as_str().unwrap());
                            }
                            e.title("Caturday");
                            e.colour(0xFFFFFF);
                            e
                        })
                    })
                    .await
                {
                    println!("Error sending message: {:?}", why);
                }
                if item["is_video"].as_bool().unwrap()
                    && !item["media"]["reddit_video"]["is_gif"].as_bool().unwrap()
                {
                    if let Err(why) = ChannelId(CONFIG["Channel_id"].as_u64().unwrap())
                        .say(
                            &ctx.http,
                            item["media"]["reddit_video"]["fallback_url"]
                                .as_str()
                                .unwrap(),
                        )
                        .await
                    {
                        println!("Error sending message: {:?}", why);
                    }
                }
                time = time
                    + Duration::milliseconds(
                    (86400000 / (CONFIG["Per_day"].as_u64().unwrap() - 1)) as i64,
                );
                sleep(std::time::Duration::from_millis(
                    (time.timestamp_millis() - Utc::now().timestamp_millis()) as u64,
                ));
            }
        }
    }
}
