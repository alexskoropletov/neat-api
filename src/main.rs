use warp::{serve, Filter};

// each module represents an entire API route
mod store;
mod banks;

#[tokio::main]
async fn main() {
    let store_routes = store::get_routes();
    let banks_routes = banks::get_routes();

    serve(store_routes.or(banks_routes))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
