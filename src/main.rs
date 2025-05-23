mod commands;
use poise::serenity_prelude as serenity;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use dotenv::dotenv;

pub struct Data {
    pub citation_channels: RwLock<HashMap<serenity::GuildId, serenity::ChannelId>>,
    pub select_roles: RwLock<HashMap<serenity::GuildId, Vec<serenity::RoleId>>>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            citation_channels: RwLock::new(HashMap::new()),
            select_roles: RwLock::new(HashMap::new()),
        }
    }
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Arc<Data>, Error>;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();
    let data = Arc::new(Data::new());

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::admin::setup(),
                commands::admin::create_channel(),
                commands::citations::citation(),
                commands::citations::citation_msg(),
                commands::utility::role(),
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(data)
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
