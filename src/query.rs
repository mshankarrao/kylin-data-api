extern crate serde;
extern crate serde_json;

use actix_web::{web::Path, HttpResponse};
use chrono::Utc;
use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{thread, time};

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
    #[serde(default)]
    result: String,
}

#[get("/get_data/{path1}/vs/{path2}")]
#[tokio::main]
pub async fn get_data(curr1: Path<(String, String)>) -> HttpResponse {
    let mut v: Vec<Price> = Vec::new();
    let mut source: HashMap<&str, String> = HashMap::new();
    let (p1, p2): (String, String) = curr1.0;
    println!("hello {}", p1);
    println!("hello {}", p2);
    source.insert(
        "coinbase",
        format!("https://api.pro.coinbase.com/products/{}-{}/ticker", p1, p2),
    );
    source.insert(
        "binance",
        format!(
            "https://api.binance.com/api/v3/ticker/price?symbol={}{}",
            p1, p2
        ),
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
    for val in v.iter() {
        let serialized = serde_json::to_string(val).unwrap();
        let deserialized: Price = serde_json::from_str(&serialized).unwrap();
        data.insert(val.source.to_string(), deserialized);
    }

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(Prices { results: data })
}

pub async fn get_helper(sources: &str, urlval: &str) -> Result<Price, ExitFailure> {
    // let ten_millis = time::Duration::from_millis(5000);
    // let now = time::Instant::now();
    // thread::sleep(ten_millis);
    let url = format!("{}", urlval);
    let url = Url::parse(&*url)?;
    let mut res = reqwest::get(url).await?.json::<Price>().await?;
    res.time = Utc::now().to_string();
    res.source = sources.to_string();

    Ok(res)
}
