mod commands;

use anyhow::Context as _;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use serenity::all::GuildId;
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;

use crate::commands::{
    greeting::greeting,
    winner::winner,
    list_channel_members::list_channel_members,
    teamup::teamup,
    test::test
};


pub struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;


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
                    teamup(),
                    test(),
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
