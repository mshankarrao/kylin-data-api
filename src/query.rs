extern crate serde;
extern crate serde_json;

use actix_web::HttpResponse;
use chrono::Utc;
use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::response::Response;

pub const APPLICATION_JSON: &str = "application/json";

type Prices = Response<Price>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Price {
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
    #[serde(default)]
    source: String,
}

#[get("/get_data")]
#[tokio::main]
pub async fn get_data() -> HttpResponse {
    let mut v: Vec<Price> = Vec::new();
    let mut source: HashMap<&str, &str> = HashMap::new();
    source.insert(
        "coinbase",
        "https://api.pro.coinbase.com/products/BTC-USDC/ticker",
    );
    source.insert(
        "binance",
        "https://api.binance.com/api/v3/ticker/price?symbol=BTCUSDC",
    );

    for (sources, url) in source.iter() {
        println!("Calling {}: {}", sources, url);
        let value = get_helper(sources, url).await;
        match value {
            Ok(val) => v.push(val),
            Err(e) => println!("error parsing header: {:?}", e),
        }
    }

    let mut data: HashMap<String, Price> = HashMap::new();
    let mut count = 1;
    for val in v.iter() {
        let serialized = serde_json::to_string_pretty(val).unwrap();
        let deserialized: Price = serde_json::from_str(&serialized).unwrap();
          data.insert("source".to_string() + &count.to_string(), deserialized);
        count = count + 1;
    }

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(Prices { results: data })
}

pub async fn get_helper(sources: &str, urlval: &str) -> Result<Price, ExitFailure> {
    let url = format!("{}", urlval);

    let url = Url::parse(&*url)?;
    let mut res = reqwest::get(url).await?.json::<Price>().await?;
    res.time = Utc::now().to_string();
    res.source = sources.to_string();

    Ok(res)
}
