use core::str::FromStr;
use serde::{Serialize, Deserialize};

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
