use dotenv::dotenv;
use std::env;
use warp::{serve, Filter};

mod common {
    pub mod errors;
    pub mod responses;
    pub mod stdout;
}
mod data_store;
// each module represents an entire API route. TODO: move to router.rs
mod auth;
mod currency;
mod health;
mod income_period;
mod user;

#[tokio::main]
async fn main() {
    common::stdout::hello();
    dotenv().ok();

    let store: data_store::Store = data_store::Store::new().await;

    let health_check_routes = health::get_routes();
    let currency_routes = currency::get_routes(store.clone()).await;
    let auth_routes = auth::get_routes(store.clone());
    let income_period_routes = income_period::get_routes();
    let user_routes = user::get_routes(store.clone()).await;

    let port: u16 = match env::var("PORT") {
        Ok(val) => val.parse::<u16>().unwrap_or(3030),
        Err(_) => {
            common::stdout::warn("PORT not set, defaulting to 3030", ());
            3030
        }
    };

    serve(
        health_check_routes
            .or(currency_routes)
            .or(auth_routes)
            .or(income_period_routes)
            .or(user_routes)
            .recover(common::errors::handle_rejection),
    )
    .run(([127, 0, 0, 1], port))
    .await
}
