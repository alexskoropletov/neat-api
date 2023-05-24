use warp::Filter;
use crate::{currency, auth::with_auth};

pub async fn get_routes() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    let store = currency::Store::new().await;
    let store_filter = warp::any().map(move || store.clone());

    let prefix = warp::path!("currency" / ..);

    let symbols = warp::get()
        .and(prefix)
        .and(warp::path("symbols"))
        .and(warp::path::end())
        .and(with_auth())
        .and(store_filter.clone())
        .and_then(currency::get_list);

    let base_currency = warp::get()
        .and(prefix)
        .and(warp::path("base-currency"))
        .and(warp::path::end())
        .and(with_auth())
        .and_then(currency::get_base_currency);

    let exchange_rates = warp::get()
        .and(prefix)
        .and(warp::path("exchange-rates"))
        .and(warp::path::end())
        .and(with_auth())
        .and(store_filter.clone())
        .and_then(currency::get_exchange_rates);

    let update_rates = warp::get()
        .and(prefix)
        .and(warp::path("update-rates"))
        .and(warp::path::end())
        .and(with_auth())
        .and(store_filter)
        .and_then(currency::update_rates);
    

    symbols
        .or(base_currency)
        .or(exchange_rates)
        .or(update_rates)
}
