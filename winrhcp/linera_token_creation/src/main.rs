mod contract;
mod views;
mod types;
mod errors;

use actix_web::{post, web, App, HttpServer, Responder, HttpResponse};
use serde::Deserialize;
use crate::contract::create_token;
use crate::types::TokenRequest;

#[post("/create_token")]
async fn create_token_endpoint(req: web::Json<TokenRequest>) -> impl Responder {
    let token_name = &req.name;
    let token_symbol = &req.symbol;
    let total_supply = req.total_supply;

    match create_token(token_name, token_symbol, total_supply).await {
        Ok(_) => HttpResponse::Ok().json("Token created successfully"),
        Err(err) => HttpResponse::BadRequest().json(format!("Error: {:?}", err)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(create_token_endpoint)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
