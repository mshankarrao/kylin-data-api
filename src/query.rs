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

// #[derive(Debug, Deserialize, Serialize)]
// pub struct APIResponse {
//     pub source: String,
//     pub created_at: DateTime<Utc>,
//     pub actual_price: i32,
// }

// impl APIResponse {
//     pub fn new(source: String) -> Self {
//         Self {
//             created_at: Utc::now(),
//             source,
//             actual_price: 100,
//         }
//     }
// }

// #[get("/get_data")]
// pub async fn get_data() -> HttpResponse {
//     let hello = "Kylin is great";

//     let mut v: Vec<APIResponse> = Vec::new();
//     v.push(APIResponse::new("Kylin".to_string()));

//     HttpResponse::Ok()
//         .content_type(APPLICATION_JSON)
//         .json(Prices { results: v })
// }

#[derive(Debug, Deserialize, Serialize)]
struct Price {
    symbol: String,
    price: String,
}

impl Price {
    async fn new(price: String, symbol: String) -> Self {
        Self {
            price: price,
            symbol: symbol,
        }
    }
}

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
    let url = format!("https://api.binance.com/api/v3/ticker/price?symbol=BTCDAI");

    let url = Url::parse(&*url)?;
    let res = reqwest::get(url).await?.json::<Price>().await?;
    //let res = Price::new(res.price, res.symbol).await;
    //println!("Bhaiya {:?}", res);

    Ok(res)
}
