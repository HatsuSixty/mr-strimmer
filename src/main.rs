use std::fs::read_to_string;
use std::process::exit;
use std::thread::spawn;

use serenity::async_trait;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::StandardFramework;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use serde::Deserialize;

use floatwidgets::widgets::*;

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

#[derive(Deserialize)]
struct DiscordConfig {
    token: String,
}

#[derive(Deserialize)]
struct CameraConfig {
    enabled: bool,
    border_color: String,
}

#[derive(Deserialize)]
struct ImageConfig {
    enabled: bool,
    image_path: String,
}

#[derive(Deserialize)]
struct TextConfig {
    enabled: bool,
    text: String,
    background_image: String,
}

#[derive(Deserialize)]
struct WidgetConfig {
    camera: CameraConfig,
    image: ImageConfig,
    text: TextConfig,
}

#[derive(Deserialize)]
struct Config {
    discord: DiscordConfig,
    widgets: WidgetConfig,
}

#[tokio::main]
async fn main() {
    let config: Config;
    if let Ok(text) = read_to_string("config.toml") {
        match toml::from_str(text.as_str()) {
            Ok(table) => {
                config = table;
            }
            Err(e) => {
                eprintln!("ERROR: Could not parse file `config.toml`\n{}", e);
                exit(1);
            }
        }
    } else {
        eprintln!("ERROR: Could read file `config.toml`");
        exit(1);
    }

    if config.widgets.image.enabled {
        spawn(|| {
            floatimg::floatimg(config.widgets.image.image_path);
        });
    }

    if config.widgets.camera.enabled {
        spawn(|| {
            floatwebcam::floatwebcam();
        });
    }

    if config.widgets.text.enabled {
        spawn(|| {
            floatext::floatext(
                "assets/Cantarell.ttf".to_string(),
                config.widgets.text.text,
                config.widgets.text.background_image,
            );
        });
    }

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "!"
        .help(&HELP_CMD)
        .group(&COMMANDS_GROUP);

    let token = config.discord.token.as_str();
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
