use parking_lot::RwLock;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use core::str::{FromStr};

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

#[derive(Clone, Debug)]
pub struct Store {
  currency: Arc<RwLock<Items>>
}

impl Store {
    pub fn new() -> Self {
        let store = Store {
            currency: Arc::new(RwLock::new(Vec::new())),
        };
        store.currency.write().push(Item { code: Currency::USD, name: "United States Dollar".to_string() });
        store.currency.write().push(Item { code: Currency::EUR, name: "Euro".to_string() });
        store.currency.write().push(Item { code: Currency::RUB, name: "Russian Ruble".to_string() });

        store
    }
}

pub async fn get_list(
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let result = store.currency.read();
        Ok(warp::reply::json(&*result))
}
