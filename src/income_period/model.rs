use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum IncomePeriod {
    Once,
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

pub async fn get_income_periods() -> Result<impl warp::Reply, warp::Rejection> {

        let income_periods = vec![
            IncomePeriod::Once,
            IncomePeriod::Daily,
            IncomePeriod::Weekly,
            IncomePeriod::Monthly,
            IncomePeriod::Yearly,
        ];
    
        Ok(warp::reply::json(&income_periods))
}

