use warp::Filter;
use crate::health;
use crate::auth::with_auth;

pub fn get_routes() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    let prefix = warp::path!("health" / ..);
    
    warp::get()
        .and(prefix)
        .and(with_auth())
        .and(warp::path::end())
        .and_then(health::check)
}
