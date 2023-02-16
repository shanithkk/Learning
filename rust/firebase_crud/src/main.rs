use std::collections::HashMap;

use firebase_rs::*;
use serde::{Deserialize,Serialize};

#[tokio::main]
async fn main() {
    let firebase = Firebase::new("<firebase url>").unwrap();

    let user = User{
        username: "testuser".to_string(),
        password: "testpassword".to_string(),
        fullname: "testfullname".to_string(),
    };
    let fire = firebase.at("users");
    let s = fire.set(&user).await;
    println!("{:?}", s);

    let get = fire.get::<HashMap<String, User>>().await;
    println!("{:?}", get);
}

#[derive(Debug, Serialize, Deserialize)]
struct User{
    username: String,
    password: String,
    fullname: String,
}