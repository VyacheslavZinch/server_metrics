use std::{collections::HashMap, env};
use dotenv;
use redis::*;

async fn conn_str() -> String {
    dotenv::dotenv().ok();

    let pwd = env::var("REDIS_PASSWD").expect("THE REDIS PASSWORD IS UNAVAILABLE");
    let usr = env::var("REDIS_USR").expect("THE REDIS USER IS UNAVAILABLE");
    let host = env::var("REDIS_HOST").expect("INCORRECT REDIS ADDRESS");

    let conn_str = format!("redis://{}:{}@{}:6380/0", usr, pwd, host);
    conn_str
}

pub async fn get_auth_datas() -> HashMap<String, String> {

    let conn_str = conn_str().await;
    let client = Client::open(conn_str);
    let conn = client.unwrap().get_multiplexed_async_connection().await;

    let data: HashMap<String, String> = conn.unwrap().get("auth_data").await.unwrap();

    data

}
pub async fn get_auth_tokens() -> Vec<String>{
    let mut result = Vec::<String>::new();
    let conn_str = conn_str().await;
    let client = Client::open(conn_str);
    let conn = client.unwrap().get_multiplexed_async_connection().await;

    result
}
pub async fn set_new_token(){}

