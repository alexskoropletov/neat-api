use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{data_store::Store, common::stdout};

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

pub type Items = HashMap<String, Item>;


pub async fn initiate(store: Store) -> Option<Store> {
    let id = "12345".to_string();
    store.users.write().insert(id.clone(), Item {
        id,
        name: "Admin".to_string(),
        email: "admin@ur.be".to_string(),
        login: "admin".to_string(),
        password: "admin".to_string(),
    });
    stdout::debug("Users", &store.users.read());

    Some(store)
}
