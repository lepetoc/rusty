use std::vec;

use poise::serenity_prelude::{self as serenity, PermissionOverwrite, Permissions};

use crate::{Context, Error};

///Placeholder help text
#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn create_channel(
    ctx: Context<'_>,
    #[description = "Nom du channel"] name: String,
    #[description = "Nom du role"] role_name: Option<String>,
) -> Result<(), Error> {
    let role_name = role_name.as_ref().unwrap_or_else(|| &name);
    let guild = ctx.guild().as_deref().unwrap().to_owned();
    let builder = serenity::EditRole::new()
        .name(role_name)
        .hoist(false)
        .mentionable(true);
    let role = guild.create_role(ctx.http(), builder).await?;
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            ctx.say("This command must be used in a server").await?;
            return Ok(());
        }
    };
    let permissions = vec![
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL,
            deny: Permissions::SEND_TTS_MESSAGES,
            kind: serenity::PermissionOverwriteType::Role(role.id),
        },
        PermissionOverwrite {
            allow: Permissions::SEND_MESSAGES,
            deny: Permissions::VIEW_CHANNEL,
            kind: serenity::PermissionOverwriteType::Role(guild_id.everyone_role()),
        },
    ];

    let builder = serenity::CreateChannel::new(&name)
        .kind(serenity::ChannelType::Text)
        .permissions(permissions);
    let channel = guild.create_channel(ctx.http(), builder).await;

    match channel {
        Ok(channel) => {
            let message = serenity::CreateMessage::new()
                .content(format!("Ce salon a été créé par {}", ctx.author()));
            channel.send_message(ctx.http(), message).await?;
            // ctx.say(format!("Channel {} created successfully", channel))
            //     .await?;
            ctx.send(
                poise::CreateReply::default()
                    .content(format!("Channel {} created successfully", channel))
                    .ephemeral(true),
            )
            .await?;
        }
        Err(err) => {
            ctx.say(format!("Failed to create channel: {}", err))
                .await?;
        }
    }
    Ok(())
}

/// Configure the channel where citations will be saved
#[poise::command(
    slash_command,
    required_permissions = "ADMINISTRATOR",
    subcommands("setup_citation"),
    subcommand_required
)]
pub async fn setup(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, rename = "citation")]
pub async fn setup_citation(
    ctx: Context<'_>,
    #[description = "Channel to save citations"] channel: serenity::GuildChannel,
) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            ctx.say("This command must be used in a server").await?;
            return Ok(());
        }
    };

    {
        let mut citation_channels = ctx.data().citation_channels.write().await;
        citation_channels.insert(guild_id, channel.id);
    }

    ctx.send(
        poise::CreateReply::default()
            .content(format!("Citations will now be saved to {}", channel))
            .ephemeral(true),
    )
    .await?;
    Ok(())
}
