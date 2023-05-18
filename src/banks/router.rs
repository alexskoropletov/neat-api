use warp::Filter;
use crate::banks;

fn json_body() -> impl Filter<Extract = (banks::Item,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn delete_json() -> impl Filter<Extract = (banks::Id,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn post_json() -> impl Filter<Extract = (banks::Item,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn get_routes() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    let store = banks::Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let prefix = warp::path!("banks" / "items" / ..);

    let add_items = warp::post()
        .and(prefix)
        .and(warp::path::end())
        .and(json_body())
        .and(store_filter.clone())
        .and_then(banks::update_grocery_list);

    let get_items = warp::get()
        .and(prefix)
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(banks::get_grocery_list);

    let delete_item = warp::delete()
        .and(prefix)
        .and(warp::path::end())
        .and(delete_json())
        .and(store_filter.clone())
        .and_then(banks::delete_grocery_list_item);

    let update_item = warp::put()
        .and(prefix)
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter)
        .and_then(banks::update_grocery_list);

    add_items.or(get_items).or(delete_item).or(update_item)
}
