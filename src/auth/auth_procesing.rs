use rand::{distributions::Alphanumeric, Rng};

use super::redis::get_auth_datas;
use crate::auth::auth_types::auth_pwd::*;

pub async fn set_new_token() {
    let mut rng = rand::thread_rng();
    let new_token = (0..16)
        .map(|_| char::from(rng.sample(Alphanumeric)))
        .collect::<String>();
}

pub async fn verify_token() {}

pub async fn verify_login_passwd(data: BasicAuth) -> bool {
    let auth_data = get_auth_datas().await;
    let (login, pwd) = (data.username, data.password);

    if auth_data.contains_key(&login) {
        if auth_data.get(&login).unwrap() == &pwd {
            return true;
        }
    }
    false
}
