use crate::{Context, Error};

/// Greet command caller
#[poise::command(slash_command)]
pub async fn greeting(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(format!("hello {}", ctx.author().name)).await?;
    Ok(())
}
