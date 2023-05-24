use core::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::data_store::Store;

#[derive(Debug, Deserialize, Serialize, Clone, Hash, Eq, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
pub enum Currency {
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
#[allow(non_snake_case)]
pub struct ExchangeRate {
    pub fromCurrency: Currency,
    pub toCurrency: Currency,
    pub rate: f64,
}

pub type ExchangeRates = Vec<ExchangeRate>;

pub async fn initiate(store: Store) -> Option<Store> {
    store.currency.write().push(Item {
        code: Currency::USD,
        name: "United States Dollar".to_string(),
    });
    store.currency.write().push(Item {
        code: Currency::EUR,
        name: "Euro".to_string(),
    });
    store.currency.write().push(Item {
        code: Currency::RUB,
        name: "Russian Ruble".to_string(),
    });

    update_rates(store).await
}

pub async fn update_rates(store: Store) -> Option<Store> {
    store
        .rates
        .write()
        .insert((Currency::USD, Currency::EUR), 0.85);
    store
        .rates
        .write()
        .insert((Currency::EUR, Currency::USD), 1.18);
    store
        .rates
        .write()
        .insert((Currency::USD, Currency::RUB), 73.0);
    store
        .rates
        .write()
        .insert((Currency::RUB, Currency::USD), 0.014);
    store
        .rates
        .write()
        .insert((Currency::EUR, Currency::RUB), 86.0);
    store
        .rates
        .write()
        .insert((Currency::RUB, Currency::EUR), 0.012);
    store
        .rates
        .write()
        .insert((Currency::EUR, Currency::EUR), 1.0); // lol
    store
        .rates
        .write()
        .insert((Currency::USD, Currency::USD), 1.0);
    store
        .rates
        .write()
        .insert((Currency::RUB, Currency::RUB), 1.0);
    Some(store)
}
