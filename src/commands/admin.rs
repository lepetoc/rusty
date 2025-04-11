use std::vec;

use poise::serenity_prelude as serenity;

use crate::{Context, Error};

///A command that create a private channel with the associated role
#[poise::command(slash_command, required_permissions = "ADMINISTRATOR", ephemeral)]
pub async fn create_channel(
    ctx: Context<'_>,
    #[description = "Nom du channel"] name: String,
    #[description = "Nom du role"] role_name: Option<String>,
    #[description = "Category"]
    #[channel_types("Category")]
    category: Option<serenity::ChannelId>,
) -> Result<(), Error> {
    let guild = ctx.guild().as_deref().unwrap().to_owned();
    let role_name = role_name.as_ref().unwrap_or_else(|| &name);
    let role_builder = serenity::EditRole::new()
        .name(role_name)
        .hoist(false)
        .mentionable(true);
    let role = guild.create_role(ctx.http(), role_builder).await?;
    let permissions = vec![
        serenity::PermissionOverwrite {
            allow: serenity::Permissions::VIEW_CHANNEL,
            deny: serenity::Permissions::empty(),
            kind: serenity::PermissionOverwriteType::Role(role.id),
        },
        serenity::PermissionOverwrite {
            allow: serenity::Permissions::empty(),
            deny: serenity::Permissions::VIEW_CHANNEL,
            kind: serenity::PermissionOverwriteType::Role(guild.id.everyone_role()),
        },
    ];

    let mut channel_builder = serenity::CreateChannel::new(&name)
        .kind(serenity::ChannelType::Text)
        .permissions(permissions);

    if let Some(category_id) = category {
        channel_builder = channel_builder.category(category_id);
    }
    let channel = guild.create_channel(ctx.http(), channel_builder).await;

    match channel {
        Ok(channel) => {
            let message = serenity::CreateMessage::new()
                .content(format!("Ce salon a été créé par {}", ctx.author()));
            channel.send_message(ctx.http(), message).await?;
            ctx.say(format!("Channel {} created successfully", channel))
                .await?;
        }
        Err(err) => {
            ctx.say(format!("Failed to create channel: {}", err))
                .await?;
        }
    }
    Ok(())
}

#[poise::command(
    slash_command,
    required_permissions = "ADMINISTRATOR",
    subcommands("setup_citation"),
    subcommand_required
)]
pub async fn setup(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Configure the channel where citations will be saved
#[poise::command(slash_command, rename = "citation", ephemeral)]
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
            .content(format!("Citations will now be saved to {}", channel)),
    )
    .await?;
    Ok(())
}
