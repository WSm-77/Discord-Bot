use serenity::all::{ChannelId, Member};

use crate::{Context, Error};

pub async fn get_channel_members(
    guild_id: poise::serenity_prelude::GuildId,
    voice_channel_id: ChannelId,
    ctx: Context<'_>
) -> Result<Vec<Member>, Error> {
    let guild = ctx.cache()
        .guild(guild_id)
        .ok_or("Guild not found")?;

    Ok(guild.voice_states.values()
        .filter(|voice_state| voice_state.channel_id == Some(voice_channel_id))
        .filter_map(|voice_state| guild.members.get(&voice_state.user_id))
        .cloned()
        .collect()
    )
}

pub async fn get_voice_channels(
    guild_id: poise::serenity_prelude::GuildId,
    ctx: Context<'_>
) -> Result<Vec<poise::serenity_prelude::GuildChannel>, Error> {
    let guild = ctx.cache()
        .guild(guild_id)
        .ok_or("Guild not found")?;

    let channels = guild.channels
        .values()
        .filter(|channel| channel.kind == serenity::model::channel::ChannelType::Voice)
        .cloned()
        .collect();

    Ok(channels)
}
