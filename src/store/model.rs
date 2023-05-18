use warp::http;
use parking_lot::RwLock;
use std::{collections::HashMap};
use std::sync::Arc;
use serde::{Serialize, Deserialize};

use crate::common::Currency;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Id {
    id: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Item {
    id: i32,
    name: String,
    person_id: i32,
    currency: Currency,
    amount: f64,
    /* 
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    person_id INTEGER NOT NULL REFERENCES person(id),
    currency CHARACTER(3) REFERENCES currency(code),
    amount NUMERIC(10,2) NOT NULL
    */
}


pub type Items = HashMap<i32, Item>;

#[derive(Clone, Debug)]
pub struct Store {
  goal: Arc<RwLock<Items>>
}

impl Store {
    pub fn new() -> Self {
        Store {
            goal: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

pub async fn update_goal(
    item: Item,
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        store.goal.write().insert(item.id,item);

        Ok(warp::reply::with_status(
            "Added items to the goal list",
            http::StatusCode::CREATED,
        ))
}

pub async fn delete_list_item(
    id: Id,
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        store.goal.write().remove(&id.id);


        Ok(warp::reply::with_status(
            "Removed item from goal list",
            http::StatusCode::OK,
        ))
}

pub async fn get_list(
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let result = store.goal.read();
        Ok(warp::reply::json(&*result))
}

// pub async fn get_key_value(id: Id, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
//     match store.goal.read().get(&id.id) {
//         Some(item) => Ok(warp::reply::json(&item)),
//         None => Err(warp::reject::not_found()),
//     }
// }
