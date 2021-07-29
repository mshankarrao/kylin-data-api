use actix_web::web::{Data, Json, Path};
use actix_web::{web, HttpResponse};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use std::env;

use crate::response::Response;

pub const APPLICATION_JSON: &str = "application/json";

pub type Prices = Response<Price>;

#[derive(Debug, Deserialize, Serialize)]
struct Price {
    #[serde(default)]
    symbol: String,
    #[serde(default)]
    bid: String,
    #[serde(default)]
    price: String,
    #[serde(default)]
    volume: String,
    #[serde(default)]
    time: String,
}

// impl Price {
//     async fn new(price: String, symbol: String) -> Self {
//         Self {
//             price: price,
//             symbol: symbol,
//         }
//     }
// }

#[get("/get_data")]
#[tokio::main]
pub async fn get_data() -> HttpResponse {
    let mut v: Vec<Price> = Vec::new();
    let value = get_helper().await;
    match value {
        Ok(val) => v.push(val),
        Err(e) => println!("error parsing header: {:?}", e),
    }

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(Prices { results: v })
}

pub async fn get_helper() -> Result<Price, ExitFailure> {
    let url = format!("https://api.binance.com/api/v3/ticker/price?symbol=BTCUSDC");

    let url = Url::parse(&*url)?;
    let mut res = reqwest::get(url).await?.json::<Price>().await?;
    res.time = Utc::now().to_string();
    //res.time
    //let res = Price::new(res.price, res.symbol).await;
    //println!("Bhaiya {:?}", res);

    Ok(res)
}
