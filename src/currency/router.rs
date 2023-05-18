use warp::Filter;
use crate::currency;

pub fn get_routes() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    let store = currency::Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let prefix = warp::path!("currency" / ..);

    warp::get()
        .and(prefix)
        .and(warp::path("symbols"))
        .and(warp::path::end())
        .and(store_filter)
        .and_then(currency::get_list)    
}
