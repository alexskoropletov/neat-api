use dotenv::dotenv;
use std::env;
use warp::{serve, Filter};

// each module represents an entire API route
mod banks;
mod store;
// mod common;
mod currency;

#[tokio::main]
async fn main() {
    dotenv().ok();
    /*
    mod common;
    GET /health

    mod auth;
    POST /auth/login

    mod account;
    GET /account
    GET /account/:id
    DELETE /account/:id
    POST /account

    mod currency;
    + GET /currency/symbols
    GET /currency/base-currency
    GET /currency/exchange-rates
    GET /currency/update-rates

    mod distribution-by-account;
    GET /distribution-by-account
    GET /distribution-by-account/:id
    DELETE /distribution-by-account/:id
    POST /distribution-by-account

    mod distribution-by-source-of-income;
    GET /distribution-by-source-of-income
    GET /distribution-by-source-of-income/:id
    DELETE /distribution-by-source-of-income/:id
    POST /distribution-by-source-of-income
    GET /distribution-by-source-of-income/year/:year

    mod goal;
    GET /goal
    GET /goal/:id
    DELETE /goal/:id
    POST /goal

    mod income-period;
    GET /income-period

    mod user;
    GET /user
    GET /user/me

    mod source-of-income;
    GET /source-of-income
    GET /source-of-income/:id
    DELETE /source-of-income/:id
    POST /source-of-income
    GET /source-of-income/year/:year
    */
    let currency_routes = currency::get_routes();
    let store_routes = store::get_routes();
    let banks_routes = banks::get_routes();

    let port: u16 = match env::var("PORT") {
        Ok(val) => val.parse::<u16>().unwrap_or(3030),
        Err(_) => 3030,
    };

    serve(
        currency_routes
            .or(store_routes)
            .or(banks_routes)
        )
        .run(([127, 0, 0, 1], port))
        .await
}
