use super::utils::utils::select_voice_channels;
use crate::{Context, Error};

/// Command for test purpose
#[poise::command(slash_command)]
pub async fn test(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id()
        .ok_or("Couldn't find guild")?;

    let selected_voice_channels_ids = select_voice_channels(guild_id, &ctx).await?;

    let selected_voice_channels_names: Vec<String> = {
        let guild = ctx
            .guild()
            .ok_or("Command must be called on the server")?;

        selected_voice_channels_ids
            .iter()
            .filter_map(|cid| guild.channels.get(cid))
            .map(|ch| ch.name.clone())
            .collect()
    };

    ctx.say(format!("Selected channels: {}",
        selected_voice_channels_names.join(", ")
    )).await?;

    Ok(())
}
