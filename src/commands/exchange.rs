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

// todo technically, should this be called every x period, to sync data, or could it even be called on each function use?
/// Init to build database of exchange rates
pub fn init() {
    let data_dir: PathBuf = env::current_dir()
        .map(|home_dir| home_dir.join(".moneyman"))
        .expect("need a home directory");

    // Fetches the historical data from European Central Bank, and creates an
    // exchange store.
    match ExchangeStore::sync(data_dir) {
        Ok(_) => {},
        Err(e) => print!("Error syncing exchange store: {}", e)
    }
}


/// Commands
#[poise::command(slash_command)]
pub async fn exchange(
    ctx: Context<'_>,
    #[description = "Convert currency between different valutas"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    #[min = 0]
    amount: i64,
    #[choices("USD", "EUR", "DKK", "SEK", "GBP", "CAD")]
    from: &'static str,
    #[choices("USD", "EUR", "DKK", "SEK", "GBP", "CAD")]
    to: &'static str,
) -> Result<(), Error> {

    // load the store
    let data_dir: PathBuf = env::current_dir()
        .map(|home_dir| home_dir.join(".moneyman"))
        .expect("need a home directory");

    let store_result = ExchangeStore::open(data_dir);

    // if error, we respond with error...
    if let Err(err) = store_result {
        ctx.say(format!["Failed to open currency store, err: {}", err]).await
            .expect("Failed to send err message");
        return Ok(())
    }

    // store is not err, so we can unwrap
    let store = store_result.unwrap();

    // get currencies
    let from_currency = iso::find(from).expect("failed to find currency");
    let to_currency = iso::find(to).expect("failed to find currency");

    // get current money input
    let input = Money::from_decimal(Decimal::new(amount, 2), from_currency);

    let latest_date = store.get_latest_date();
    let date = Utc::now().naive_utc().date().checked_sub_days(Days::new(1)).unwrap();

    let converted_money = store.convert_on_date(input, to_currency, date);

    ctx.say(format!("{:?}{} is {}{}", amount, from_currency.iso_alpha_code, 00, to_currency.iso_alpha_code)).await?;
    // ctx.say(format!("{:?}{} is {}{}", amount, from_currency.iso_alpha_code, converted_money.amount(), converted_money.currency().iso_alpha_code)).await?;


    // else we are good, and can continue

    // let embed = CreateEmbed::default()
    //     .colour(Colour::from_rgb(70, 199, 244))
    //     .title("Digby's War Room")
    //     .description("Status of the Arma server")
    //     .field("Modpack", body.modpack, false)
    //     .field("Map Name", body.map_name, true)
    //     .field("Template", body.template, true)
    //     .field("","", false)
    //     .field("Player Count", format!("{}/{}", body.player_num, body.player_max), true)
    //     .field("Mod Count", body.mods.len().to_string(), true)
    //     .footer(CreateEmbedFooter::new(format!("Requested at: {}", Local::now())));
    // .author(CreateEmbedAuthor::new(
    //     format!("requested by: {}", capitalize(&ctx.author().name)))
    //     .icon_url(ctx.author().avatar_url().unwrap_or("".to_string()))
    // );

    // let reply = CreateReply::default().embed(embed).reply(true);
    let reply = CreateReply::default().reply(true);

    ctx.send(reply).await?;
    Ok(())
}