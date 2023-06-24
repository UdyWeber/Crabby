mod commands;

use std::collections::HashSet;
use std::env::var;

use dotenv::dotenv;

use serenity::async_trait;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::StandardFramework;
use serenity::http::Http;
use serenity::model::prelude::{Message, Ready, ResumedEvent};
use serenity::prelude::*;
use tracing::info;

use crate::commands::idea::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, event: ResumedEvent) {
        info!("Resumed event: {:?}", event);
    }

    async fn message(&self, _ctx: Context, msg: Message) {
        match msg.author.bot {
            true => info!("Skipping message from bot: {}", msg.author.name),
            false => info!("Received message from {}: {}", msg.author.name, msg.content),
        }
    }
}

#[group]
#[commands(idea)]
struct General;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let token = var("DISCORD_TOKEN").expect("Token is invalid");

    info!("Token Loaded!");

    let serenity_http = Http::new(&token);

    let (owners, _bot_id) = match serenity_http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let intents = GatewayIntents::all();

    let framework = StandardFramework::new()
        .configure(|bot_configuration| bot_configuration.prefix("!").owners(owners))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    info!("Created Client :D");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        info!("An error occurred while running the client: {:?}", why);
    }
}
