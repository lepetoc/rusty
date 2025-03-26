use poise::serenity_prelude as serenity;

use crate::{Context, Error};
/// Save the text as a citation
#[poise::command(slash_command)]
pub async fn citation(
    ctx: Context<'_>,
    #[description = "Citation de génie"] citation: String,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());
    let embed = create_citation_embed(
        &user,
        &citation,
        Some(format!("Sauvegardé par {}", ctx.author().name,)),
        None,
    );

    let message = serenity::CreateMessage::new().embed(embed);

    // Get the configured channel ID
    let channel_id = get_citation_channel(ctx).await?;

    // Send the message to the configured channel
    channel_id.send_message(ctx.http(), message).await?;
    ctx.say("Citation sauvegardée").await?;
    Ok(())
}

/// Save the selected message to a specific channel
#[poise::command(context_menu_command = "Citation")]
pub async fn citation_msg(
    ctx: Context<'_>,
    #[description = "Citation de génie"] msg: serenity::Message,
) -> Result<(), Error> {
    let timestamp = msg.timestamp.format("%d/%m/%Y %H:%M");
    let user = msg.author;
    let embed = create_citation_embed(
        &user,
        &msg.content,
        Some(format!(
            "Sauvegardé par {} • Message original du {}",
            ctx.author().name,
            timestamp
        )),
        Some(msg.channel_id),
    );
    let message = serenity::CreateMessage::new().embed(embed);
    // Get the configured channel ID
    let channel_id = get_citation_channel(ctx).await?;

    // Send the message to the configured channel
    channel_id.send_message(ctx.http(), message).await?;
    ctx.say("Citation sauvegardée").await?;
    Ok(())
}

async fn get_citation_channel(ctx: Context<'_>) -> Result<serenity::ChannelId, Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            return Err("This command must be used in a server".into());
        }
    };

    // Try to get the configured channel ID for this guild
    let citation_channels = ctx.data().citation_channels.read().await;

    match citation_channels.get(&guild_id) {
        Some(channel_id) => Ok(*channel_id),
        None => Err("No citation channel configured. Use /setup first.".into()),
    }
}

fn create_citation_embed(
    user: &serenity::User,
    content: &str,
    footer_text: Option<String>,
    channel_id: Option<serenity::ChannelId>,
) -> serenity::CreateEmbed {
    let mut embed = serenity::CreateEmbed::new()
        .title("Citation")
        .description(format!("{} a dit : {}", user, content))
        .thumbnail(
            user.static_avatar_url()
                .unwrap_or_else(|| user.default_avatar_url()),
        );

    if let Some(footer) = footer_text {
        embed = embed.footer(serenity::CreateEmbedFooter::new(footer));
    }

    if let Some(channel) = channel_id {
        embed = embed.field("Canal d'origine", format!("<#{}>", channel), true);
    }

    embed
}
