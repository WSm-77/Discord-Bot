use poise::serenity_prelude::*;

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

pub async fn select_voice_channels_menu(
    guild_id: poise::serenity_prelude::GuildId,
    ctx: &Context<'_>
) -> Result<(), Error> {

    let options: Vec<CreateSelectMenuOption>;

    {
        let guild = ctx.cache()
            .ok_or("Coulnd't retrive context cache")?
            .guild(guild_id)
            .ok_or("Guild not found")?;

        options = guild.channels.values()
            .filter(|channel| channel.kind == poise::serenity_prelude::ChannelType::Voice)
            .map(|channel| poise::serenity_prelude::CreateSelectMenuOption::new(channel.name.clone(), channel.id.to_string()))
            .collect();
    }

    let options_len = options.len() as u8;

    if options_len < 2 {
        ctx.say("Not enough voice channels to select from.").await?;
        return Ok(());
    }

    let select_menu_custom_id = "Select voice channels".to_string();

    let menu: CreateSelectMenu = CreateSelectMenu::new(select_menu_custom_id.clone(), CreateSelectMenuKind::String { options: options } )
        .placeholder("Select voice channels")
        .min_values(2)
        .max_values(options_len);

    ctx.send(poise::CreateReply::default()
        .components(vec![CreateActionRow::SelectMenu(menu)])
    ).await?;


    let interaction: Option<ComponentInteraction> = {
        let author_id = ctx.author().id;
        let select_menu_custom_id = select_menu_custom_id.clone();

        ComponentInteractionCollector::new(ctx.serenity_context().shard.clone())
            .custom_ids(vec![select_menu_custom_id])
            .author_id(author_id)
            .timeout(std::time::Duration::from_secs(60))
            .await
    };

    if let Some(interaction) = interaction {
        interaction.create_response(ctx.serenity_context(),
        CreateInteractionResponse::UpdateMessage(CreateInteractionResponseMessage::default()))
        .await?;

        let selected_channel_ids: Vec<poise::serenity_prelude::ChannelId> = match &interaction.data.kind {
            poise::serenity_prelude::ComponentInteractionDataKind::StringSelect{values} => {
                values.iter()
                    .filter_map(|id_str| id_str.parse::<u64>().ok())
                    .map(ChannelId::new)
                    .collect()
            }
            _ => {
                ctx.say("Unexpected component interaction type.").await?;
                return Ok(());
            }
        };


        let channel_names: Vec<String> = {
            let guild = ctx
                .cache()
                .ok_or("Cache unavailable")?
                .guild(guild_id)
                .ok_or("Guild not in cache")?;

            selected_channel_ids
                .iter()
                .filter_map(|cid| guild.channels.get(cid))
                .map(|ch| ch.name.clone())
                .collect()
        };

        ctx.say(format!(
            "You selected: {}",
            channel_names.join(", ")
        )).await?;
    }
    else {
        ctx.say("Timeout reached without selection!!!").await?;
    }

    Ok(())
}
