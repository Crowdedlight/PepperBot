use serde::{Deserialize, Deserializer};
use serenity::builder::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter};
use tracing_subscriber::fmt::format;
use crate::{Context, Error};
use serenity::model::Colour;
use chrono::{DateTime, Local};
use poise::CreateReply;
use serde_json::{Value};
use serde_with::{serde_as, BoolFromInt};
use crate::helpers::capitalize;

/// Is it friday?
///
/// Is it friday? Truely?
#[poise::command(prefix_command, slash_command, aliases("friday?"))]
pub async fn friday(
    ctx: Context<'_>,
    #[description = "Is it friday?"]
    command: Option<String>,
) -> Result<(), Error> {

    // todo check if its actual friday and change response?

    ctx.say("No").await?;
    Ok(())
}
