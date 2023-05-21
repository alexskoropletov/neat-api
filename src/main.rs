use dotenv::dotenv;
use std::env;
use warp::{serve, Filter};

// each module represents an entire API route
mod banks;
mod store;
mod jwt;
mod currency;
mod health;
mod income_period;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let health_check_routes = health::get_routes();
    let currency_routes = currency::get_routes().await;
    let store_routes = store::get_routes();
    let banks_routes = banks::get_routes();
    let jwt_routes = jwt::get_routes();
    let income_period_routes = income_period::get_routes();

    let port: u16 = match env::var("PORT") {
        Ok(val) => val.parse::<u16>().unwrap_or(3030),
        Err(_) => 3030,
    };

    serve(
        health_check_routes
            .or(currency_routes)
            .or(store_routes)
            .or(banks_routes)
            .or(jwt_routes)
            .or(income_period_routes)
        )
        .run(([127, 0, 0, 1], port))
        .await
}
