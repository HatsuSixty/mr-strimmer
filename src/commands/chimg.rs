use super::*;

use floatwidgets::widgets::floatimg;

#[command]
async fn chimg(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    println!("[chimg] Running command...");

    if args.is_empty() {
        msg.reply(ctx, "ERROR: No image was provided").await?;
    } else {
        let image = args.single_quoted::<String>().unwrap();
        msg.reply(ctx, format!("Changing image to `{}`...", image).as_str())
            .await?;
        floatimg::change_img(image);
    }
    Ok(())
}
