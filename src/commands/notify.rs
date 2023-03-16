use super::*;

use notify_rust::Notification;

#[command]
async fn notify(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    println!("[notify] Running command...");

    msg.reply(ctx, "Annoying streamer with a notification...")
        .await?;
    while let Ok(x) = args.single_quoted::<String>() {
        Notification::new()
            .summary(format!("`{}` says:", msg.author.name).as_str())
            .body(x.as_str())
            .show()?;
    }
    msg.channel_id.say(&ctx.http, "Done :XD:").await?;
    Ok(())
}
