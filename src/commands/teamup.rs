use poise::serenity_prelude::*;
use rand::{self, seq::SliceRandom};

use crate::{Context, Error};
use super::utils::utils::*;

use std::time::Duration;

async fn parse_channels(channels: String, voice_channels: Vec<GuildChannel>) -> Result<Vec<ChannelId>, Error> {
    // get voice channels by String
    let vc_names: Vec<_> = channels
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    let number_of_teams = vc_names.len();
    if number_of_teams <= 1 {
        return Err("Need at least two teams to perfom teamup.".into());
    }

    let mut voice_channels_teams = vec![];
    for name in vc_names {
        let voice_channel_team = voice_channels
            .iter()
            .find(|channel| channel.name == *name)
            .ok_or_else(|| format!("Voice channel '{}' not found", name))?;

        voice_channels_teams.push(voice_channel_team.id);
    }

    Ok(voice_channels_teams)
}

#[poise::command(slash_command)]
pub async fn teamup(
    ctx: Context<'_>,
    #[description = "Comma-separated list of voice channels for teams"]
    channels: Option<String>
) -> Result<(), Error> {
    // get guild attributes
    let guild_id = ctx.guild_id().ok_or("Command must be used in the server")?;
    let author = ctx.author();
    let voice_channel_id = ctx
        .guild()
        .and_then(|g| g.voice_states.get(&author.id)?.channel_id)
        .ok_or("Command must be used in the server")?;

    // get channel members
    let mut channel_members = get_channel_members(guild_id, voice_channel_id, ctx).await?;

    // use only real members, exclude bots
    channel_members.retain(|member| !member.user.bot);

    // get all voice channels
    let voice_channels = get_voice_channels(guild_id, ctx).await?;

    let voice_channels_teams = match channels {
        Some(channels) => {
            parse_channels(channels, voice_channels).await?
        },
        None => {
            select_voice_channels(guild_id, &ctx).await?
        }
    };

    let number_of_teams = voice_channels_teams.len();

    // get number of members
    let number_of_members = channel_members.len();
    if number_of_members <= 1 {
        return Err("Need at least two members in the voice channel to perfom teamup.".into());
    }
    if number_of_members < number_of_teams {
        return Err("Number of members in a channel must be at least the amount of teams to perfom teamup".into());
    }

    let original_members = channel_members.clone();

    loop {
        // Reset channel_members from original for each iteration
        let mut current_members = original_members.clone();
        
        // shuffle randomly channel members
        {
            let mut rng = rand::rng();
            current_members.shuffle(&mut rng);
        }
        
        // perform teamup
        let mut teams: Vec<Vec<Member>> = vec![vec![]; number_of_teams];
        for (i, member) in current_members.into_iter().enumerate() {
            let team_index = i % number_of_teams;
            teams[team_index].push(member.clone());
        }
        
        // create an embed for reponses
        let mut embed = CreateEmbed::new()
            .title(format!("Splitted {} users into {} teams", number_of_members, number_of_teams))
            .color(0x00D700); // Gold color

        // send embed message with results
        for (i, team) in teams.iter().enumerate() {
            let team_name = format!("Team {}", i + 1);
            let members_list = team
                .iter()
                .map(|m| m.display_name().to_string())
                .collect::<Vec<_>>()
                .join("\n");
    
            embed = embed.field(team_name, members_list, true);
        }

        let msg = ctx.send(
            poise::CreateReply::default()
                .embed(embed.clone())
                .components(vec![CreateActionRow::Buttons(vec![
                    CreateButton::new("accept").label("✅ Accept").style(ButtonStyle::Success),
                    CreateButton::new("reshuffle").label("🔁 Reshuffle").style(ButtonStyle::Primary),
                ])])
        ).await?;

        let message_id = msg.message().await?.id;

        if let Some(mci) = ComponentInteractionCollector::new(ctx.serenity_context())
            .author_id(ctx.author().id)
            .message_id(message_id)
            .timeout(Duration::from_secs(60))
            .await
        {
            match mci.data.custom_id.as_str() {
                "accept" => {
                    mci.create_response(ctx.serenity_context(), CreateInteractionResponse::UpdateMessage(
                            CreateInteractionResponseMessage::new()
                                .embed(embed.clone())
                                .components(vec![])
                        )
                    ).await?;
                    
                    // distribute team members to voice channels
                    for (i, team) in teams.iter().enumerate() {
                        let team_voice_channel = voice_channels_teams[i];
                        for member in team {
                            member.move_to_voice_channel(ctx.serenity_context(), team_voice_channel).await?;
                        }
                    }

                    ctx.send(poise::CreateReply::default()
                        .content("✅ Teams confirmed and members moved.")
                        .ephemeral(true)
                    ).await?;

                    return Ok(());
                },
                "reshuffle" => {
                    mci.create_response(
                        ctx.serenity_context(),CreateInteractionResponse::Defer(
                            CreateInteractionResponseMessage::new()
                                .embed(embed.clone())
                                .components(vec![])
                            )
                    ).await?;
                    continue;
                },
                _ => {}
            }
        }
        else {
            ctx.send(poise::CreateReply::default()
                .content("⏳ No response. Cancelled teamup.")
                .ephemeral(true)
            ).await?;
            break;
        }
    };

    Ok(())
}
