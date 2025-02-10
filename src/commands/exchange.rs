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


/// Exchange valuta between currencies
///
/// Simply give the amount and then select the currency it is in, and what currency you want
///
/// Example:
/// `~/exchange 120 EUR USD`
#[poise::command(slash_command)]
pub async fn exchange(
    ctx: Context<'_>,
    #[description = "Convert currency between different valutas"]
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
    let input = Money::from_decimal(Decimal::from(amount), from_currency);

    let latest_date = store.get_latest_date().unwrap_or_default();

    let converted_money = store.convert_on_date(input, to_currency, latest_date)?;

    // else we are good, and can continue

    let embed = CreateEmbed::default()
        .colour(Colour::from_rgb(70, 199, 244))
        .field("Input", format!("{:?} {}", amount, from_currency.iso_alpha_code), false)
        .field("Output", format!("{} {}", converted_money.amount().round_dp(2), to_currency.iso_alpha_code), false)
        .footer(CreateEmbedFooter::new(format!("Requested at: {}", Local::now())));

    let reply = CreateReply::default().embed(embed).reply(true);

    ctx.send(reply).await?;
    Ok(())
}