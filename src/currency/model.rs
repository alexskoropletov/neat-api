use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use core::str::{FromStr};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Currency {
    Usd,
    Eur,
    Rub,
}

impl FromStr for Currency {
    type Err = ();

    fn from_str(input: &str) -> Result<Currency, Self::Err> {
        match input {
            "USD" => Ok(Currency::Usd),
            "EUR" => Ok(Currency::Eur),
            "RUB" => Ok(Currency::Rub),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Id {
    code: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Item {
    code: String,
    name: Currency,
}

pub type Items = HashMap<Currency, Item>;

#[derive(Clone, Debug)]
pub struct Store {
  currency: Arc<RwLock<Items>>
}

impl Store {
    pub fn new() -> Self {
        Store {
            currency: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

pub async fn get_list(
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let result = store.currency.read();
        Ok(warp::reply::json(&*result))
}
