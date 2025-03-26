// use poise::serenity_prelude as serenity;

use crate::{Context, Error};

///Placeholder help text
#[poise::command(slash_command)]
pub async fn role(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Failed to create channel").await?;
    Ok(())
}
