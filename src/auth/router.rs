use crate::{auth, data_store};
use warp::Filter;

fn json_body() -> impl Filter<Extract = (auth::Auth,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn get_routes(
    store: data_store::Store,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let store_filter = warp::any().map(move || store.clone());

    let prefix = warp::path!("auth" / ..);

    warp::post()
        .and(prefix)
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(store_filter)
        .and(json_body())
        .and_then(auth::get_token)
}
