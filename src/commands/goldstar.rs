use serde::{Deserialize, Deserializer};
use serenity::builder::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter};
use tracing_subscriber::fmt::format;
use crate::{Context, Error};
use serenity::model::Colour;
use chrono::{DateTime, Local};
use poise::CreateReply;
use serde_json::{Value};
use serde_with::{serde_as, BoolFromInt};
use serenity::all::{RoleId, UserId};
use crate::helpers::capitalize;
use rand::seq::SliceRandom;

/// Do you have a gold star?
///
/// This command is only for those that have a certified real gold star!
#[poise::command(prefix_command, slash_command, user_cooldown = 30)]
pub async fn goldstar(
    ctx: Context<'_>,
    #[description = "Do you have a gold star?"]
    command: Option<String>,
) -> Result<(), Error> {


    let good_boi = ["https://tenor.com/view/youre-a-wonderful-human-youre-a-great-person-youre-fantastic-youre-amazing-youre-awesome-gif-15328827",
        "https://tenor.com/view/good-boy-good-boi-good-boiiii-good-dog-labrador-gif-8148443075840194090",
        "https://tenor.com/view/slickric-naturedoge69-gif-23058228",
        "https://tenor.com/view/coach-josh-wood-coach-josh-gold-star-gold-star-gif-593378152944786379"];

    let bad = [
        "https://tenor.com/view/no-please-no-god-no-stop-please-stop-gif-17190431",
        "https://tenor.com/view/water-spray-stop-sprinkle-water-gif-12639877674493562634",
        "https://tenor.com/view/periodicazo-fifidonia-newspaper-gif-5642703893973833213",
        "https://tenor.com/view/newspaper-gif-18150652"
    ];

    let reply = match ctx.author_member().await.unwrap_or_default().roles.contains(&RoleId::new(1112517192169308213)) {
        true => good_boi.choose(&mut rand::thread_rng()).unwrap().to_string(),
        false => bad.choose(&mut rand::thread_rng()).unwrap().to_string(),
    };

    // xcom override
    let actual_reply = match ctx.author().id.get() {
        225022116678991872 => "Welcome Commander!\nhttps://tenor.com/view/xcom-logo-vigilo-confido-gif-15525933".to_string(), // Salt
        227840954705510400 => "Hi Wind!\nhttps://tenor.com/view/flag-denmark-gif-europe-gif-27417972".to_string(), // Wind
        123367387113193475 => "Yo Crow!\nhttps://tenor.com/view/cat-drone-gif-20814552".to_string(), // Crow
        177145737874898944 => "No, you can't ban them Bolem!\nhttps://tenor.com/view/fire-it-crowd-moss-technical-gif-9562579".to_string(), // Bolem
        _ => reply
    };

    ctx.say(actual_reply).await?;
    Ok(())
}
