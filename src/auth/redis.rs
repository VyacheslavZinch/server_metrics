
pub fn conn_str() -> String {
    dotenv().ok();

    let pwd = env::var("REDIS_PASSWD").expect("THE REDIS PASSWORD IS UNAVAILABLE");
    let usr = env::var("REDIS_USR").expect("THE REDIS USER IS UNAVAILABLE");
    let host = env::var("REDIS_HOST").expect("INCORRECT REDIS ADDRESS");

    let conn_str = format!("redis://{}:{}@{}:6380/0", usr, pwd, host);
}

