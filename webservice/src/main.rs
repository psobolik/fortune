/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-05-02
 */
mod fortune_config;

#[macro_use]
extern crate rocket;

use std::path::PathBuf;

use rocket::http::Status;
use rocket::serde::json::Json;

use crate::fortune_config::FortuneConfig;
use shared_library::{Fortune, FortuneStats};

#[get("/")]
async fn index() -> Result<Json<Fortune>, Status> {
    let path = get_data_path()?;
    match shared_library::random_fortune(&path).await {
        Ok(fortune) => Ok(Json(fortune)),
        _ => Err(Status::InternalServerError),
    }
}
#[get("/info")]
async fn info() -> Result<Json<Vec<FortuneStats>>, Status> {
    let path = get_data_path()?;
    match shared_library::fortune_stats(&path).await {
        Ok(fortune_file_infos) => Ok(Json(fortune_file_infos)),
        _ => Err(Status::InternalServerError),
    }
}

fn get_data_path() -> Result<PathBuf, Status> {
    match FortuneConfig::new() {
        Ok(config) => Ok(config.data_path),
        _ => Err(Status::InternalServerError),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, info])
}
