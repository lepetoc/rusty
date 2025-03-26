mod commands;
// mod types;
use poise::serenity_prelude as serenity;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// Data struct to hold bot configuration
pub struct Data {
    // Maps guild_id -> citation_channel_id
    pub citation_channels: RwLock<HashMap<serenity::GuildId, serenity::ChannelId>>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            citation_channels: RwLock::new(HashMap::new()),
        }
    }
}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Arc<Data>, Error>;

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();
    let data = Arc::new(Data::new());

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::citation(),
                commands::citation_msg(),
                // commands::create_channel(),
                commands::setup(),
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
