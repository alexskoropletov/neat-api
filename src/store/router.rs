use warp::Filter;
use crate::store;

fn json_body() -> impl Filter<Extract = (store::Item,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn delete_json() -> impl Filter<Extract = (store::Id,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn post_json() -> impl Filter<Extract = (store::Item,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn get_routes() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    let store = store::Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let prefix = warp::path!("goal" / ..);

    let add_items = warp::post()
        .and(prefix)
        .and(warp::path::end())
        .and(json_body())
        .and(store_filter.clone())
        .and_then(store::update_goal);

    let get_items = warp::get()
        .and(prefix)
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(store::get_list);

    let delete_item = warp::delete()
        .and(prefix)
        .and(warp::path::end())
        .and(delete_json())
        .and(store_filter.clone())
        .and_then(store::delete_list_item);

    let update_item = warp::put()
        .and(prefix)
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter)
        .and_then(store::update_goal);

    add_items.or(get_items).or(delete_item).or(update_item)
}
