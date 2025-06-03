mod commands;

use anyhow::Context as _;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use serenity::all::{GuildId, Member};
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;
use rand::{self, seq::SliceRandom};

use crate::commands::{
    greeting::greeting,
    winner::winner,
    list_channel_members::list_channel_members,
    utils::utils::get_channel_members
};


pub struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;


#[poise::command(slash_command)]
async fn teamup(
    ctx: Context<'_>,
    #[description = "Comma-separated list of voice channels for teams"]
    channels: String
    
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

    // get voice channels by String
    let vc_names: Vec<_> = channels
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    // get number of teams for users to be splitted to
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

    // get number of members
    let number_of_members = channel_members.len();
    if number_of_members <= 1 {
        return Err("Need at least two members in the voice channel to perfom teamup.".into());
    }
    if number_of_members < number_of_teams {
        return Err("Number of members in a channel must be at least the amount of teams to perfom teamup".into());
    }

    // shuffle randomly channel members
    {
        let mut rng = rand::rng();
        channel_members.shuffle(&mut rng);
    };

    // perform teamup
    let mut teams: Vec<Vec<Member>> = vec![vec![]; number_of_teams];
    for (i, member) in channel_members.into_iter().enumerate() {
        let team_index = i % number_of_teams;
        teams[team_index].push(member);
    }

    // distribute team members to voice channels
    for (i, team) in teams.iter().enumerate() {
        let team_voice_channel = voice_channels_teams[i];
        for member in team {
            member.move_to_voice_channel(ctx.serenity_context(), team_voice_channel).await?;
        }
    }

    // send embed message with results
    let mut embed = CreateEmbed::new()
        .title(format!("Splitted {} users into {} teams", number_of_members, number_of_teams))
        .color(0x00D700); // Gold color

    for (i, team) in teams.iter().enumerate() {
        let team_name = format!("Team {}", i + 1);
        let members_list = team
            .iter()
            .map(|m| m.display_name().to_string())
            .collect::<Vec<_>>()
            .join("\n");

        embed = embed.field(team_name, members_list, true);
    }

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}

async fn get_voice_channels(
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

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
    .get("DISCORD_TOKEN")
    .context("'DISCORD_TOKEN' was not found")?;

    // Get server id set in `Secrets.toml`
    let guild_id: GuildId = secret_store
        .get("GUILD_ID")
        .context("'GUILD_ID' was not found")?
        .parse()
        .context("Couldn't parse 'GUILD_ID' string into GuildId object")?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                    list_channel_members(),
                    greeting(),
                    winner(),
                    teamup()
                ],
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(ctx, &framework.options().commands, guild_id).await?;
                Ok(Data {})
            })
        })
        .build();

    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::GUILD_MEMBERS;

    let client = ClientBuilder::new(discord_token, intents)
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}
