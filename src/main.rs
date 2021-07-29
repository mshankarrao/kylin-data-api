#[macro_use]
extern crate actix_web;

use std::{io};

use actix_web::{App, HttpServer};

mod query;
mod response;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // register HTTP requests handlers
            .service(query::get_data)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}
