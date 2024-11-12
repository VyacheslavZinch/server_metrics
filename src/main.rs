use std::default;

use rocket::serde::json::Json;
use rocket::{catch, catchers, get, routes};
use rocket_dyn_templates::serde::json::serde_json;
use rocket_dyn_templates::serde::json::Value;
use serde::Deserialize;
use serde_json::json;
use sys_metrics::sys_metrics::SysInfo;
use sys_metrics::sys_metrics::{
    AllData, Component, CpuInfo, Disk, MemInfo, NetworkInterface, OutputData, Process,
};

mod auth;
mod sys_metrics;
mod utils;

#[get("/")]
async fn get_status() -> Json<AllData> {
    AllData::new().json_data()
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

#[get("/storage")]
async fn get_storage() -> Json<Vec<Disk>> {
    let data = Disk::get_discs();
    Disk::json_data(data)
}

#[get("/processes")]
async fn get_processes() -> Json<Vec<Process>> {
    let data = Process::get_processes();
    Process::json_data(data)
}

#[get("/components")]
async fn get_components() -> Json<Vec<Component>> {
    let data = Component::get_components();
    Component::json_data(data)
}

/////////////////////////////////////////////////////////////////////////////////////////////
async fn get_unit_logs() {}

async fn get_warnings() {}

async fn get_alerts() {}

/////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Deserialize)]
struct AppConfig {
    id: Option<usize>,
    port: u16,
}

#[catch(404)]
async fn not_found() -> Json<String> {
    Json("NOT FOUND".to_string())
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let routes = routes![
        login,
        get_sysinfo,
        get_cpu,
        get_memory,
        get_network,
        get_status,
        get_storage,
        get_components,
        get_processes
    ];

    let _rocket = rocket::build()
        .mount("/", routes)
        .register("/", catchers![not_found])
        .register("/sysinfo", catchers![not_found])
        .register("/cpu", catchers![not_found])
        .register("/memory", catchers![not_found])
        .register("/network", catchers![not_found])
        .register("/login", catchers![not_found])
        .register("/storage", catchers![not_found])
        .register("/processes", catchers![not_found])
        .register("/components", catchers![not_found])
        .launch()
        .await;

    Ok(())
}
