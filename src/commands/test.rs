use super::utils::utils::select_voice_channels_menu;
use crate::{Context, Error};

/// Greet command caller
#[poise::command(slash_command)]
pub async fn test(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id()
        .ok_or("Couldn't find guild")?;

    select_voice_channels_menu(guild_id, &ctx).await
}
