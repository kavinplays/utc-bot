use std::env;
// use dotenv::dotenv;
use serenity::{
    async_trait,
    model::{
        gateway::{
            Ready,
            Activity,
        },
        user::OnlineStatus,
        channel::Message,
    },
    prelude::*,
};
use tokio::time::{sleep, Duration};
use chrono::Utc;
extern crate reqwest;

struct Handler;


#[async_trait]
impl EventHandler for Handler {

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
        else if msg.content == "!time" {
            let now = Utc::now();
            let formatted_time = now.format("%H:%M (%d/%m)").to_string();
            if let Err(why) = msg.channel_id.say(&ctx.http, formatted_time).await {
                println!("Error sending message: {:?}", why);
            }
        }
        else if msg.content == "!localtime" {
            let meh = Utc::now().timestamp().to_string();
            let localtime = "<t:".to_owned() + &meh + &">".to_owned();
            println!("{}",&meh);
            if let Err(why) = msg.channel_id.say(&ctx.http, localtime).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready){
        println!("{} is connected!", ready.user.name);
        
        let mut check_time: i32 = 5;
        while check_time > 0 {
            let time_now = Utc::now();
            let temp = time_now.format("%S").to_string();
            check_time = temp.parse().unwrap();
            println!("{}",check_time);
            sleep(Duration::from_millis(1000)).await;
        }
        println!("hurray!");

        loop{
            let time_now = Utc::now();
            let formatted_time = time_now.format("%H:%M (%d/%m)").to_string();
            let activity = Activity::playing(&formatted_time);
            let status = OnlineStatus::Online;
            ctx.set_presence(Some(activity), status).await;
            sleep(Duration::from_millis(60000)).await;
        }
    }
}

#[tokio::main]
async fn main() {
    // dotenv().expect(".env file not found");
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // The Application Id is usually the Bot User Id.
    let application_id: u64 = env::var("APPLICATION_ID")
        .expect("Expected an application id in the environment")
        .parse()
        .expect("application id is not a valid id");

    // Build our client.
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .application_id(application_id)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
