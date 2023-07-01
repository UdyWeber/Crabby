use std::fmt::{Display, Formatter, Result};
use std::str::FromStr;
use std::time::Duration;

use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[derive(Debug, PartialEq)]
enum FeelingStatus {
    Happy,
    Sad,
    OK,
}

impl FromStr for FeelingStatus {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Happy" => Ok(FeelingStatus::Happy),
            "Sad" => Ok(FeelingStatus::Sad),
            "OK" => Ok(FeelingStatus::OK),
            _ => Err(()),
        }
    }
}

impl Display for FeelingStatus {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

#[command]
pub async fn feel(ctx: &Context, msg: &Message) -> CommandResult {
    let feeling_message = msg
        .channel_id
        .send_message(
            &ctx.http,
            |message: &mut serenity::builder::CreateMessage<'_>| {
                message
                    .embed(|e| {
                        e.title(format!("Hey fellow user {}...", msg.author.name));
                        e.description("How you're feeling today buddy?")
                    })
                    .components(|c| {
                        c.create_action_row(|row| {
                            row.create_select_menu(|menu| {
                                // Every component has to have a unique custom_id or it wont work cause the
                                // HTTP call to discord.com will fail
                                menu.custom_id("mood_select");
                                menu.placeholder("Choose how you're feeling today!");
                                menu.options(|f| {
                                    f.create_option(|o| {
                                        o.label(FeelingStatus::Happy)
                                            .description("I'm feeling happy today!")
                                            .value(FeelingStatus::Happy)
                                    });
                                    f.create_option(|o| {
                                        o.label(FeelingStatus::Sad)
                                            .description("I'm feeling blue today...")
                                            .value(FeelingStatus::Sad)
                                    });
                                    f.create_option(|o| {
                                        o.label(FeelingStatus::OK)
                                            .description("I'm feeling neutral today!")
                                            .value(FeelingStatus::OK)
                                    })
                                })
                            })
                        })
                    })
            },
        )
        .await
        .unwrap();

    match feeling_message
        .await_component_interaction(&ctx)
        .timeout(Duration::from_secs(10))
        .await
    {
        Some(interaction) => {
            let interaction_data = FeelingStatus::from_str(&interaction.data.values[0]).unwrap();

            match interaction_data {
                FeelingStatus::Happy => feeling_message
                    .reply(&ctx, "Good that you're feeling fine today :D")
                    .await
                    .unwrap(),
                FeelingStatus::OK => feeling_message
                    .reply(&ctx, "Keep on going buddy")
                    .await
                    .unwrap(),
                FeelingStatus::Sad => {
                    let sad_message = feeling_message
                        .reply(&ctx, "To bad... tell me a little about what you're feeling")
                        .await
                        .unwrap();

                    match &msg.author.await_reply(&ctx).timeout(Duration::from_secs(600)).await {
                        Some(answer) => {
                            answer.reply(&ctx, "I'll communicate this to the managers for you... Just don't forget to stay hidrated, see the sun for a while and exercise!").await.unwrap()
                        },
                        None => {
                            sad_message.reply(&ctx, "It's okay to not communicating just seek for help if needed :`)").await.unwrap()
                        },
                    }
                }
            }
        }
        None => feeling_message.reply(&ctx, "Timed out").await.unwrap(),
    };

    feeling_message.delete(&ctx).await.unwrap();

    Ok(())
}
