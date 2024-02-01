use std::env;

use reqwest::header::ACCEPT;
use reqwest::Response;
use serde_derive::Deserialize;
use serenity::async_trait;
use serenity::gateway::ActivityData;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, Configuration, CommandResult};

#[group]
#[commands(
    ping,
    date,
    dadjoke,
    // bus
)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: serenity::model::gateway::Ready) {

        ctx.set_activity(Some(ActivityData::playing("H- hiiii~~ v0.0.1")));

        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new().group(&GENERAL_GROUP);
    framework.configure(Configuration::new().prefix("!")); // set the bot's prefix to "~"

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");

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

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "P-pOwOng! ").await?;
    msg.react(ctx, '👉').await?;
    msg.react(ctx, '👈').await?;

    Ok(())
}

#[command]
async fn date(ctx: &Context, msg: &Message) -> CommandResult {
    let date = chrono::Local::now();
    msg.reply(ctx, format!("The current date and time is: {}", date)).await?;
    print!("The current date and time is: {}", date);
    Ok(())
}

#[derive(Deserialize)]
struct JokeResponse {
    // id: String,
    joke: String,
    // status: u16,
}

async fn dadjoke_api() -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();

    let res: Response = client
        .get("https://icanhazdadjoke.com/ ")
        .header(ACCEPT, "application/json")
        .send()
        .await?;

    println!("Status: {}", res.status());

    let joke: JokeResponse = match res.json().await {
        Ok(joke) => joke,
        Err(_) => JokeResponse {
            // id: String::new(),
            joke: "Error".to_string(),
            // status: 0,
        },
    };

    return Ok(joke.joke);
}

#[command]
async fn dadjoke(ctx: &Context, msg: &Message) -> CommandResult {

    let joke = dadjoke_api().await?;

    println!("Joke:\n{}", joke);

    msg.reply(ctx, joke).await?;
    msg.react(ctx, '😀').await?;
    msg.react(ctx, '😃').await?;
    msg.react(ctx, '😄').await?;
    msg.react(ctx, '😁').await?;
    msg.react(ctx, '😆').await?;
    msg.react(ctx, '😅').await?;
    msg.react(ctx, '😂').await?;
    msg.react(ctx, '🤣').await?;
    msg.react(ctx, '🥲').await?;
    msg.react(ctx, '🥹').await?;

    Ok(())
}


//TODO print joke with this api https://dragon.best/api/brexit_bus.jpg?text=lol
// #[command]
// async fn bus(ctx: &Context, msg: &Message) -> CommandResult {
//     let joke = dadjoke_api().await?;

//     println!("Joke:\n{}", joke);


// }