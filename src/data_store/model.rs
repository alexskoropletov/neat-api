use crate::data_store::{currency, user};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Store {
    pub users: Arc<RwLock<user::Items>>,
    pub currency: Arc<RwLock<currency::Items>>,
    pub rates: Arc<RwLock<HashMap<(currency::Currency, currency::Currency), f64>>>,
}

impl Store {
    pub async fn new() -> Self {
        let mut store = Store {
            currency: Arc::new(RwLock::new(Vec::new())),
            rates: Arc::new(RwLock::new(HashMap::new())),
            users: Arc::new(RwLock::new(HashMap::new())),
        };

        let result = currency::initiate(store.clone()).await;
        if let Some(updated_store) = result {
            println!("[>] Default store complete {:?}", updated_store);
            store = updated_store;
        } else {
            println!("[!] Failed to update default store");
        }

        let result = user::initiate(store.clone()).await;
        if let Some(updated_store) = result {
            println!("[>] Default store complete {:?}", updated_store);
            store = updated_store;
        } else {
            println!("[!] Failed to update default store");
        }

        store
    }

    pub fn rates_to_json(&self) -> currency::ExchangeRates {
        let mut result = Vec::new();

        let rates = self.rates.read();
        for (key, value) in rates.iter() {
            result.push(currency::ExchangeRate {
                fromCurrency: key.0.to_owned(),
                toCurrency: key.1.to_owned(),
                rate: *value,
            });
        }

        result
    }
}
