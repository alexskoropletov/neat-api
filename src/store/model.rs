use warp::http;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use serde::{Serialize, Deserialize};

pub type Items = HashMap<String, i32>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Id {
    name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Item {
    name: String,
    quantity: i32,
}

#[derive(Clone, Debug)]
pub struct Store {
  grocery_list: Arc<RwLock<Items>>
}

impl Store {
    pub fn new() -> Self {
        Store {
            grocery_list: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

pub async fn update_grocery_list(
    item: Item,
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        store.grocery_list.write().insert(item.name, item.quantity);


        Ok(warp::reply::with_status(
            "Added items to the grocery list",
            http::StatusCode::CREATED,
        ))
}

pub async fn delete_grocery_list_item(
    id: Id,
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        store.grocery_list.write().remove(&id.name);


        Ok(warp::reply::with_status(
            "Removed item from grocery list",
            http::StatusCode::OK,
        ))
}

pub async fn get_grocery_list(
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let result = store.grocery_list.read();
        Ok(warp::reply::json(&*result))
}
