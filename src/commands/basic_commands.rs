use super::*;

use std::collections::HashSet;

use serenity::framework::standard::macros::help;
use serenity::framework::standard::{
    help_commands, CommandGroup, HelpOptions,
};
use serenity::model::prelude::UserId;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    println!("[ping] Running command...");
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[help]
async fn help_cmd(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    println!("[help_cmd] Running command...");
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}
