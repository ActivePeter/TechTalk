extern crate core;

pub mod apis;
pub mod readconfig;
pub mod memstate_lock;
// <<<<<<< HEAD
pub mod memstate_nolock;
pub mod models;
pub mod services;
pub mod db;
pub mod test;
// =======
// >>>>>>> af4c70b49831f559438f519f7fd9c6ce40425809

use std::collections::HashMap;
use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use axum::error_handling::HandleErrorLayer;
use crate::memstate_lock::MemStateWithLock;
use crate::services::verifycode::G_VERIFY_MANAGER;
use crate::test::verifycode_test::verifycode_check;

#[tokio::main]
async fn main() {
    env_logger::init();
    verifycode_check().await;
   // prepare database
    let config=readconfig::ServerConfig::read_from_file().await;
    log::debug!("The addr read from config.json : {}",config.addr);
    db::sql::sqlstart(&config).await.unwrap();
    services::init_all().await;

    let mut app = Router::new()
        .route("/", get(root))
        .route("/user_create", post(apis::user_create::create_user))
        .route("/user_login",post(apis::user_login::user_login))
        .route("/user_basic_info",post(apis::user_basic_info::user_basic_info))
        .route("/verify_token",post(apis::verify_token::verify_token))
        ;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}



