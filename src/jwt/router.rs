use warp::Filter;
use crate::jwt;

fn json_body() -> impl Filter<Extract = (jwt::Auth,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn get_routes() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    let prefix = warp::path!("auth" / ..);
    
    warp::post()
        .and(prefix)
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(json_body())
        .and_then(jwt::get_token)
}
