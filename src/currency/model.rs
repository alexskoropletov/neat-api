use crate::data_store::{Store, Currency, ExchangeRate};
use crate::data_store::currency;

pub async fn get_list(_uid: String, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    let result = store.currency.read();
    println!("[>] Currencies {:?}", result);
    Ok(warp::reply::json(&*result))
}

pub async fn get_base_currency(_uid: String) -> Result<impl warp::Reply, warp::Rejection> {
    println!("[>] Base Currencies {:?}", Currency::USD);
    Ok(warp::reply::json(&Currency::USD))
}

pub async fn get_exchange_rates(
    _uid: String,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let response = store.rates_to_json();
    println!("[>] Exchange rates {:?}", response);
    Ok(warp::reply::json(&response))
}

pub async fn update_rates(_uid: String, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    println!("[>] Updating rates");
    let result = currency::update_rates(store).await;
    if let Some(store) = result {
        let response: Vec<ExchangeRate> = store.rates_to_json();
        println!("[>] Updated rates [response] {:?}", response);
        Ok(warp::reply::json(&response))
    } else {
        println!("[!] Error updating rates");
        Ok(warp::reply::json(&"Error updating rates"))
    }
}
