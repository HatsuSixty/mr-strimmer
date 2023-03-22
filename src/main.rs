use std::fs::read_to_string;
use std::process::exit;
use std::thread::spawn;

use serenity::async_trait;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::StandardFramework;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use floatwidgets::widgets::*;

use toml::Table;

mod commands;

use commands::basic_commands::*;
use commands::chimg::*;
use commands::notify::*;

#[group]
#[commands(ping, notify, chimg)]
struct Commands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let toml: Table;
    if let Ok(text) = read_to_string("config.toml") {
        if let Ok(table) = text.parse::<Table>() {
            toml = table;
        } else {
            eprintln!("ERROR: Could not parse file `config.toml`");
            exit(1);
        }
    } else {
        eprintln!("ERROR: Could read file `config.toml`");
        exit(1);
    }

    spawn(|| {
        floatimg::floatimg(
            "/media/roberto/HomePartition/Pictures/Cursed Emojis/the spiffing brit.png".to_string(),
        );
    });

    spawn(|| {
        floatwebcam::floatwebcam();
    });

    spawn(|| {
        floatext::floatext(
            "assets/Cantarell.ttf".to_string(),
            "Hello, World!".to_string(),
            String::new(),
        );
    });

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "!"
        .help(&HELP_CMD)
        .group(&COMMANDS_GROUP);

    let token = toml["discord_token"].as_str().unwrap();
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
