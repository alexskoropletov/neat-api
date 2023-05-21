use warp::Filter;
use crate::income_period;


pub fn get_routes() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    let prefix = warp::path!("income-period" / ..);
    
    warp::get()
        .and(prefix)
        .and(warp::path::end())        
        .and_then(income_period::get_income_periods)
}
