use crate::{Context, Error};

/// Show this help menu
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "Pepper bot for our needs",
            ..Default::default()
        },
    )
        .await?;
    Ok(())
}


#[poise::command(
    prefix_command,
    slash_command,
)]
pub async fn ping(
    ctx: Context<'_>,
    #[description = "ping"]
    command: Option<String>,
) -> Result<(), Error> {

    ctx.say("Pong!").await?;
    Ok(())
}