use std::env::var;
use std::time::Duration;

use dotenv::dotenv;

use rand::Rng;
use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};
use tokio::time::sleep;


#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let framework = StandardFramework::new()
        .configure(|bot_configuration| bot_configuration.prefix("!")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = var("DISCORD_TOKEN").expect("Token is invalid");

    println!("Token Loaded!");

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    println!("Created Client :D");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let mut analyzing_message = msg.channel_id.send_message(
        &ctx.http,
        |message|{
            message.embed(|e| {
                e.title("Analyzing your idea...")

            })
        }
    ).await?;

    sleep(Duration::from_secs(5)).await;

    analyzing_message.edit(
        &ctx, |message| {
            message.embed(|e|{
                e.title("The Council of Christmas has decided...");
                
                let vector = vec![true, false];
                let index = rand::thread_rng().gen_range(0..vector.len());
                let random_value = vector[index];
                
                let x = "Lick balls";

                let decided = match random_value {
                    true => format!("Your idea has been accepted you can start: {}", x),
                    false => format!("Your idea has been denied, **YOU'RE TRASH**"),
                };

                e.description(decided);

                let from_caids = match random_value {
                    true => "Nah he couldn't think that",
                    false => "Definitely"
                };

                e.field("Came from Caids?", from_caids, true)
            })
        }
    ).await?;

    Ok(())
}