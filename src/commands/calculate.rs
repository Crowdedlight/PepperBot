use moneyman::ExchangeStore;
use std::env;
use std::path::{Path, PathBuf};
use chrono::{Days, Local, NaiveDate, Utc};
use poise::CreateReply;
use rust_decimal::Decimal;
use rusty_money::{iso, Money};
use serenity::all::{Colour, CreateEmbed, CreateEmbedFooter};
use tracing_subscriber::fmt::format;
use crate::{Context, Error};
use rink_core::*;


/// Commands
#[poise::command(slash_command, prefix_command, track_edits)]
pub async fn calc(
    ctx: Context<'_>,
    #[description = "calculate anything with support for units. See https://rinkcalc.app/manual for options"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    input: String,
) -> Result<(), Error> {

    // pass input into rink context and reply with answer
    let mut rink_ctx = simple_context()?;
    let result = one_line(&mut rink_ctx, &input);

    let msg = result.unwrap_or_else(|error| { error });

    let embed = CreateEmbed::default()
        .colour(Colour::from_rgb(70, 199, 244))
        .field("Query", input, false)
        .field("Answer", msg, false)
        .footer(CreateEmbedFooter::new(format!("Requested at: {}", Local::now())));

    let reply = CreateReply::default().embed(embed).reply(true);

    ctx.send(reply).await?;
    Ok(())
}