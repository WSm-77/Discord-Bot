use crate::{Context, Error};

use super::utils::utils::get_channel_members;

/// Lists members present on the same channel as the command caller
#[poise::command(slash_command)]
pub async fn list_channel_members(
    ctx: Context<'_>
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Command must be used in the server")?;
    let author = ctx.author();
    let voice_channel_id = ctx.guild()
            .and_then(|g| g.voice_states.get(&author.id)?.channel_id)
            .ok_or("Command can be used only when you are on a voice channel")?;

    let channel_members = get_channel_members(guild_id, voice_channel_id, ctx).await?;

    let mut response = format!("**Users on {} channel**\n", voice_channel_id.name(ctx).await?);

    for member in channel_members {
        response.push_str(format!("- {}\n", member.display_name()).as_str());
    }

    ctx.say(response).await?;

    Ok(())
}
