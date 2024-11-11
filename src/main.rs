use rocket::serde::json::Json;
use rocket::{catch, catchers, get, launch, routes};
use rocket_dyn_templates::serde::json::serde_json;
use rocket_dyn_templates::serde::json::Value;
use serde::Deserialize;
use serde_json::json;
use sys_metrics::sys_metrics::{CpuInfo, MemInfo, NetworkInterface, OutputData};
use sys_metrics::sys_metrics::SysInfo;


mod sys_metrics;
mod auth {
    pub mod auth_procesing;
    pub mod auth_pwd;
    pub mod auth_token;
    pub mod redis;
}


#[get("/")]
async fn get_status() -> Value {
    
    json!("data")
}
#[get("/login")]
async fn login() -> Value {
    
    json!("data")
}

#[get("/sysinfo")]
async fn get_sysinfo() -> Json<SysInfo> {
    SysInfo::new().json_data()
}

#[get("/cpu")]
async fn get_cpu() -> Json<CpuInfo> {
    CpuInfo::new().json_data()
}

#[get("/memory")]
async fn get_memory() -> Json<MemInfo> {
    MemInfo::new().json_data()
}

#[get("/network")]
async fn get_network() -> Json<Vec<NetworkInterface>> {
    let data = NetworkInterface::get_network_interfaces();
    NetworkInterface::json_data(data)
    
}



#[derive(Deserialize)]
struct AppConfig {
    id: Option<usize>,
    port: u16,
}

#[catch(404)]
async fn not_found()  {
    
}


#[rocket::main]
async fn main () -> Result<(), rocket::Error>{
    let _rocket = rocket::build()
    .mount("/", routes![
        login,
        get_sysinfo,
        get_cpu,
        get_memory,
        get_network,
        get_status
    ])
    .register("/", catchers![not_found])
    .register("/sysinfo", catchers![not_found])
    .register("/cpu", catchers![not_found])
    .register("/memory", catchers![not_found])
    .register("/network", catchers![not_found])
    .register("/login", catchers![not_found])
    .launch()
    .await;

    Ok(())
}