use crate::health;
use warp::Filter;

pub fn get_routes() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let prefix = warp::path!("health" / ..);

    warp::get()
        .and(prefix)
        .and(warp::path::end())
        .and_then(health::check)
}
