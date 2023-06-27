use std::time::Duration;

use serenity::builder::CreateSelectMenuOption;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub async fn feel(ctx: &Context, msg: &Message) -> CommandResult {
    let feeling_options = vec![
        CreateSelectMenuOption::new("Happy", "I'm feeling happy today"),
        CreateSelectMenuOption::new("Sad", "I'm feeling a bit blue today"),
        CreateSelectMenuOption::new("OK", "I'm in my regular mood"),
    ];

    let feeling_message = msg.channel_id
        .send_message(
            &ctx.http,
            |message: &mut serenity::builder::CreateMessage<'_>| {
                message
                    .embed(|e| {
                        e.title(format!("Hey fellow user {}...", msg.author.name));
                        e.description(
                    "Please share your idea with me and I'll pass it to the Chrismas Council",
                )
                    })
                    .components(|c| {
                        c.create_action_row(|row| {
                            row.create_select_menu(|menu| {
                                // Every component has to have a unique custom_id or it wont work cause the
                                // HTTP call to discord.com will fail
                                menu.custom_id("mood_select");
                                menu.placeholder("Choose how you're feeling today!");
                                menu.options(|f| {
                                    f.set_options(feeling_options)
                                })
                            })
                        })
                    })
            },
        )
        .await?;
    
    let interaction = match feeling_message.await_component_interaction(&ctx).timeout(Duration::from_secs(60)).await {
        Some(interaction) => {
            feeling_message.reply(&ctx, "Blablabla").await?
        },
        None => feeling_message.reply(&ctx, "Blablabla").await?,
    };

    Ok(())
}
