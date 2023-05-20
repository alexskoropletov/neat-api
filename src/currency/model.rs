use parking_lot::RwLock;
use std::sync::Arc;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use core::str::{FromStr};
// use warp::{reply, Reply, Filter, reject, Rejection, http::StatusCode};


#[derive(Debug, Deserialize, Serialize, Clone, Hash, Eq, PartialEq)]
#[allow(clippy::upper_case_acronyms)] pub enum Currency {
    USD,
    EUR,
    RUB,
}

impl FromStr for Currency {
    type Err = ();

    fn from_str(input: &str) -> Result<Currency, Self::Err> {
        match input {
            "USD" => Ok(Currency::USD),
            "EUR" => Ok(Currency::EUR),
            "RUB" => Ok(Currency::RUB),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Id {
    code: Currency,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Item {
    code: Currency,
    name: String,
}

pub type Items = Vec<Item>;

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)] pub struct ExchangeRate {
    fromCurrency: Currency,
    toCurrency: Currency,
    rate: f64,
}

pub type ExchangeRates = Vec<ExchangeRate>;


#[derive(Clone, Debug)]
pub struct Store {
    currency: Arc<RwLock<Items>>,
    rates: Arc<RwLock<HashMap<(Currency, Currency), f64>>>,
}

/**
 
            id,
          from_currency AS "fromCurrency",
          to_currency AS "toCurrency",
          MAX(created_at) AS "createdAt",
          rate  
 */

impl Store {
    pub async fn new() -> Self {
        let mut store = Store {
            currency: Arc::new(RwLock::new(Vec::new())),
            rates: Arc::new(RwLock::new(HashMap::new())),
        };
        store.currency.write().push(Item { code: Currency::USD, name: "United States Dollar".to_string() });
        store.currency.write().push(Item { code: Currency::EUR, name: "Euro".to_string() });
        store.currency.write().push(Item { code: Currency::RUB, name: "Russian Ruble".to_string() });

        let result = update(store.clone()).await;
        if let Some(updated_store) = result {
            println!("[>] Default store complete {:?}", updated_store);
            store = updated_store;
        } else {
            println!("[!] Failed to update default store");
        }

        store
    }

    pub fn rates_to_json(&self) -> ExchangeRates {
        let mut result = Vec::new();

        let rates = self.rates.read();
        for (key, value) in rates.iter() {
            result.push(ExchangeRate {
                fromCurrency: key.0.to_owned(),
                toCurrency: key.1.to_owned(),
                rate: *value,
            });
        }


        result
    }
}

pub async fn get_list(
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let result = store.currency.read();
        println!("Currencies {:?}", result);
        Ok(warp::reply::json(&*result))
}

pub async fn get_base_currency() -> Result<impl warp::Reply, warp::Rejection> {
        Ok(warp::reply::json(&Currency::USD))
}

pub async fn get_exchange_rates(
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
    let response = store.rates_to_json();
    println!("Exchange rates {:?}", response);
    Ok(warp::reply::json(&response))
}


#[derive(Debug)]
struct InvalidParameter;

impl warp::reject::Reject for InvalidParameter {}

pub async fn update_rates(
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Updating rates");
    let result = update(store).await;
    if let Some(store) = result {
        let response: Vec<ExchangeRate> = store.rates_to_json();
        println!("Updated rates [response] {:?}", response);
        Ok(warp::reply::json(&response))
    } else {
        println!("Error updating rates");
        Ok(warp::reply::json(&"Error updating rates"))
    }
    
}

async fn update(store: Store) -> Option<Store> {
    // None
    store.rates.write().insert((Currency::USD, Currency::EUR), 0.85);
    store.rates.write().insert((Currency::EUR, Currency::USD), 1.18);
    store.rates.write().insert((Currency::USD, Currency::RUB), 73.0);
    store.rates.write().insert((Currency::RUB, Currency::USD), 0.014);
    store.rates.write().insert((Currency::EUR, Currency::RUB), 86.0);
    store.rates.write().insert((Currency::RUB, Currency::EUR), 0.012);
    store.rates.write().insert((Currency::EUR, Currency::EUR), 1.0); // lol 
    store.rates.write().insert((Currency::USD, Currency::USD), 1.0);
    store.rates.write().insert((Currency::RUB, Currency::RUB), 1.0);
    Some(store)
}