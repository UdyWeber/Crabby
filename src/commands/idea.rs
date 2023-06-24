use std::time::Duration;

use rand::Rng;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use tokio::time::sleep;

#[command]
pub async fn idea(ctx: &Context, msg: &Message) -> CommandResult {
    let mut idea_message = msg
        .channel_id
        .send_message(
            &ctx.http,
            |message: &mut serenity::builder::CreateMessage<'_>| {
                message.embed(|e| {
                    e.title(format!("Hey fellow user {}...", msg.author.name));
                    e.description(
                        "Please share your idea with me and I'll pass it to the Chrismas Council",
                    )
                })
            },
        )
        .await?;

    match &msg
        .author
        .await_reply(&ctx)
        .timeout(Duration::from_secs(20))
        .await
    {
        Some(answer) => {
            let mut analyzing_message = msg
                .channel_id
                .send_message(&ctx.http, |message| {
                    message.embed(|e| e.title("Analyzing your idea..."))
                })
                .await?;

            sleep(Duration::from_secs(5)).await;

            analyzing_message
                .edit(&ctx, |message| {
                    message.embed(|e| {
                        e.title("The Council of Christmas has decided...");

                        let vector = vec![true, false];
                        let index = rand::thread_rng().gen_range(0..vector.len());
                        let random_value = vector[index];

                        let decided = match random_value {
                            true => format!(
                                "Your idea has been accepted you can start: `{}`",
                                answer.content
                            ),
                            false => format!("Your idea has been denied, **YOU'RE TRASH**"),
                        };

                        e.description(decided);

                        let from_caids = match random_value {
                            true => "Nah he couldn't think that",
                            false => "Definitely",
                        };

                        e.field("Came from Caids?", from_caids, true)
                    })
                })
                .await?;

            Ok(())
        }
        None => {
            idea_message
                .edit(&ctx, |message| {
                    message.embed(|embed| {
                        embed.title("Shame on you!");
                        embed.description(
                            "Who do you think you're dealing with? You have made the Chrismas Council Angry!! **NOW SUFFER**",
                        )
                    })
                })
                .await?;

            Ok(())
        }
    }
}
