use warp::Filter;
use crate::{user, auth::with_auth, data_store};

pub async fn get_routes(store: data_store::Store) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    let store_filter = warp::any().map(move || store.clone());

    let prefix = warp::path!("user" / ..);

    let me = warp::get()
        .and(prefix)
        .and(warp::path("me"))
        .and(warp::path::end())
        .and(with_auth())
        .and_then(user::get_me);

    let users = warp::get()
        .and(prefix)
        .and(warp::path::end())
        .and(with_auth())
        .and(store_filter)
        .and_then(user::get_list);

    users
        .or(me)
}
