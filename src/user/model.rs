use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Id {
    id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Item {
    id: String,
    name: String,
    email: String,
    login: String,
    password: String,
}

pub type Items = HashMap<Id, Item>;

#[derive(Clone, Debug)]
pub struct Store {
    items: Arc<RwLock<Items>>,
}

impl Store {
    pub async fn new() -> Self {
        Store {
            items: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

pub async fn get_list(_uid: String, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    let result = store.items.read();
    println!("[>] Users {:?}", result);
    Ok(warp::reply::json(&*result))
}

pub async fn get_me(uid: String) -> Result<impl warp::Reply, warp::Rejection> {
    println!("[>] User {:?}", uid);
    Ok(warp::reply::json(&uid))
}
