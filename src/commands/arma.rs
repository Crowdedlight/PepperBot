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

#[serde_as]
#[derive(Deserialize, Debug)]
struct ArmaMod {
    name: String,
    #[serde(rename = "steamID")]
    steam_id: u64,
    #[serde(rename = "isDLC")]
    #[serde_as(as = "BoolFromInt")]
    is_dlc: bool,
    hash: String,
}

#[derive(Deserialize, Debug)]
struct ArmaStatus {
    #[serde(rename = "playerNum")]
    player_num: u16,
    #[serde(rename = "playerMax")]
    player_max: u16,
    players: Vec<String>,
    mods: Vec<ArmaMod>,
    #[serde(rename = "mapName")]
    map_name: String,
    template: String,
    modpack: String,
}

/// Show this help menu
#[poise::command(prefix_command, slash_command, aliases("digby?"))]
pub async fn digbyserver(
    ctx: Context<'_>,
    #[description = "Status of Digbys Arma server?"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {

    // use my webapi to get details
    let res = reqwest::get("https://digbypack.crow.ovh/api/status").await?;
    let body = res.json::<ArmaStatus>().await?;
    println!("Body:\n{:?}", body);

    let embed = CreateEmbed::default()
        .colour(Colour::from_rgb(70, 199, 244))
        .title("Digby's War Room")
        .description("Status of the Arma server")
        .field("Modpack", body.modpack, false)
        .field("Map Name", body.map_name, true)
        .field("Template", body.template, true)
        .field("","", false)
        .field("Player Count", format!("{}/{}", body.player_num, body.player_max), true)
        .field("Mod Count", body.mods.len().to_string(), true)
        .footer(CreateEmbedFooter::new(format!("Requested at: {}", Local::now())));
        // .author(CreateEmbedAuthor::new(
        //     format!("requested by: {}", capitalize(&ctx.author().name)))
        //     .icon_url(ctx.author().avatar_url().unwrap_or("".to_string()))
        // );

    let reply = CreateReply::default().embed(embed).reply(true);

    ctx.send(reply).await?;
    Ok(())
}