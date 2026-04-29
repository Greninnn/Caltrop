// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::{models::config::Config, sources::steam::get_image_for_id};
mod sources;
mod models;
#[tokio::main]
async fn main() {
    println!("{:#?}", get_image_for_id("1384160").await.get(0));

    app_launcher_lib::run();
}
