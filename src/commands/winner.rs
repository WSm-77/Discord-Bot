use anyhow::Context as _;
use rand::seq::IndexedRandom;
use serenity::all::{CreateEmbed, Mention};

use crate::{Context, Error};
use super::utils::utils::*;

const CONGRATULATIONS_GIF_URL: &str = "https://media.discordapp.net/attachments/1379075185935913001/1379093353865678958/congrats-leonardo-dicaprio.gif?ex=683fa505&is=683e5385&hm=9985763ded4578f7318e8b0dc6fe72c3b39085b385c4ebd5f8626e884cb176e4&=&width=688&height=290";

/// Pick winner from your voice channel
#[poise::command(prefix_command, slash_command)]
pub async fn winner(
    ctx: Context<'_>,
    #[description = "Event name"] event: Option<String>,
    #[description = "Prize for winner"] prize: Option<String>,
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

    let winner_mention_str = Mention::User(winner.user.id).to_string();

    let mut embed = CreateEmbed::new()
        .title("🎉 Congratulations to our Winner! 🎉")
        .description(format!("Everyone, please give a big round of applause to {} for winning our contest!", winner_mention_str))
        .color(0xFFD700)
        .image(CONGRATULATIONS_GIF_URL);

    if let Some(prize) = prize {
        embed = embed.field("Prize", prize, false);
    }

    if let Some(event) = event {
        embed = embed.field("Event", event, false);
    }

    embed = embed.timestamp(chrono::Utc::now());

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}
