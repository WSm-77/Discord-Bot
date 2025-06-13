use poise::{serenity_prelude::*, CreateReply};

use crate::{Context, Error};

pub async fn get_channel_members(
    guild_id: GuildId,
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
    guild_id: GuildId,
    ctx: Context<'_>
) -> Result<Vec<GuildChannel>, Error> {
    let guild = ctx.cache()
        .guild(guild_id)
        .ok_or("Guild not found")?;

    let channels = guild.channels
        .values()
        .filter(|channel| channel.kind == ChannelType::Voice)
        .cloned()
        .collect();

    Ok(channels)
}

pub async fn select_voice_channels(
    guild_id: GuildId,
    ctx: &Context<'_>
) -> Result<Vec<ChannelId>, Error> {

    let options: Vec<CreateSelectMenuOption> = get_voice_channels(guild_id, *ctx).await?
        .iter()
        .map(|channel| CreateSelectMenuOption::new(channel.name.clone(), channel.id.to_string()))
        .collect();

    let options_len = options.len() as u8;

    if options_len < 2 {
        return Err("Not enough voice channels to select from.".into());
    }

    let select_menu_custom_id = "Select voice channels".to_string();

    let menu: CreateSelectMenu = CreateSelectMenu::new(select_menu_custom_id.clone(), CreateSelectMenuKind::String { options: options } )
        .placeholder("Select voice channels")
        .min_values(2)
        .max_values(options_len);

    let select_menu_message = ctx.send(poise::CreateReply::default()
        .components(vec![CreateActionRow::SelectMenu(menu)])
    ).await?;

    let interaction: Option<ComponentInteraction> = ComponentInteractionCollector::new(ctx.serenity_context().shard.clone())
            .custom_ids(vec![select_menu_custom_id])
            .author_id(ctx.author().id)
            .timeout(std::time::Duration::from_secs(60))
            .await;

    select_menu_message.edit(*ctx, CreateReply::default()
        .content("Successfully collected selections")
        .components(vec![])     // clear selection menu
    ).await?;

    if let Some(interaction) = interaction {
        let selected_channel_ids: Vec<ChannelId> = match &interaction.data.kind {
            ComponentInteractionDataKind::StringSelect{values} => {
                values.iter()
                    .filter_map(|id_str| id_str.parse::<u64>().ok())
                    .map(ChannelId::new)
                    .collect()
            }
            _ => {
                return Err("Unexpected component interaction type.".into());
            }
        };

        return Ok(selected_channel_ids);
    }
    else {
        return Err("Timeout reached without selection!!!".into());
    }
}
