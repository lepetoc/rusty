use poise::serenity_prelude as serenity;

use crate::{Context, Error};

///Placeholder help text
#[poise::command(slash_command)]
pub async fn role(ctx: Context<'_>) -> Result<(), Error> {
    let roles = get_select_roles(ctx).await?;
    let select_menu = serenity::CreateSelectMenu::new(
        "roles",
        serenity::CreateSelectMenuKind::Role {
            default_roles: Some(roles),
        },
    );
    ctx.send(
        poise::CreateReply::default()
            .content("Citation sauvegardée")
            .components(vec![serenity::CreateActionRow::SelectMenu(select_menu)])
            .ephemeral(true),
    )
    .await?;
    Ok(())
}

async fn get_select_roles(ctx: Context<'_>) -> Result<Vec<serenity::RoleId>, Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            return Err("This command must be used in a server".into());
        }
    };

    // Try to get the configured roles for this guild
    let select_roles = ctx.data().select_roles.read().await;

    match select_roles.get(&guild_id) {
        Some(select_roles) => Ok(select_roles.clone()),
        None => Err("No role configured for this command".into()),
    }
}
