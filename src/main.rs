use anyhow::Context as _;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use serenity::all::{ChannelId, CreateEmbed, GuildId, Member};
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;
use rand::{self, seq::IndexedRandom};

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Greet command caller
#[poise::command(slash_command)]
async fn greeting(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(format!("hello {}", ctx.author().name)).await?;
    Ok(())
}

/// Lists members present on the same channel as the command caller
#[poise::command(slash_command)]
async fn list_channel_members(
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

/// Pick winner from your voice channel
#[poise::command(prefix_command, slash_command)]
async fn winner(
    ctx: Context<'_>
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Command must be used in the server")?;
    let author = ctx.author();
    let voice_channel_id = ctx.guild()
            .and_then(|g| g.voice_states.get(&author.id)?.channel_id)
            .ok_or("Command can be used only when you are on a voice channel")?;

    let channel_members = get_channel_members(guild_id, voice_channel_id, ctx).await?;

    // create rng object inside context block to preserve thread safety
    let winner =  {
        let mut rng = rand::rng();

        channel_members.choose(&mut rng)
            .context("Couldn't choose winner")?
    };

    let winner_name = winner.display_name();
    let prize = "🎁 $20 Amazon Gift Card";
    let event_name = "Spring Giveaway 2025";

    let embed = CreateEmbed::new()
        .title("🎉 Congratulations to our Winner! 🎉")
        .description(format!("Everyone, please give a big round of applause to **{}** for winning the **{}**!", winner_name, event_name))
        .color(0xFFD700) // Gold color
        .image("https://media.giphy.com/media/111ebonMs90YLu/giphy.gif") // Confetti GIF
        .field("Prize", prize, false)
        .field("Event", event_name, false);
        // .footer(|f| f.text("Thanks to everyone who participated! Stay tuned for more contests."))
        // .timestamp(chrono::Utc::now());

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}

async fn get_channel_members(
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
                    winner()
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
        | GatewayIntents::MESSAGE_CONTENT;

    let client = ClientBuilder::new(discord_token, intents)
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}
