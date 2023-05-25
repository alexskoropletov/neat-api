use crate::{data_store::Store, common::stdout};

pub async fn get_list(uid: String, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    let result = store.users.read();
    stdout::info("User uid", &uid);
    stdout::info("Users", &result);
    Ok(warp::reply::json(&*result))
}

pub async fn get_me(uid: String) -> Result<impl warp::Reply, warp::Rejection> {
    stdout::info("User uid", &uid);
    Ok(warp::reply::json(&uid))
}
