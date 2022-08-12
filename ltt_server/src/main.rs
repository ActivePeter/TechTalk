pub mod apis;
pub mod sql;
pub mod readconfig;
pub mod memstate_lock;
// <<<<<<< HEAD
pub mod memstate_nolock;
pub mod models;
pub mod services;
pub mod db;

#[macro_use]
extern crate lazy_static;
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

#[tokio::main]
async fn main() {
    // initialize tracing
    // tracing_subscriber::fmt::init();

    // 注册logger
    env_logger::init();

    // 读取配置
    let config=readconfig::ServerConfig::read_from_file().await;

    // 初始化token秘钥
    services::token::init_from_config(config.token_secret.clone());

    // 启动数据库
    log::debug!("The addr read from config.json : {}",config.addr);
    sql::sqlstart(&config).await.unwrap();

    //LetTeachSome1@#
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/user_create", post(apis::user_create::create_user))
        // Add middleware to all routes
        // .layer(
        //     ServiceBuilder::new()
        //         // Handle errors from middleware
        //         .layer(HandleErrorLayer::new(handle_error))
        //         // .load_shed()
        //         // .concurrency_limit(1024)
        //         // .timeout(Duration::from_secs(10))
        //         // .layer(TraceLayer::new_for_http())
        //         // .layer(Extension(SharedState::default()))
        //         .into_inner(),
        // )
        ;

        // .route("/user_login",post())
        // .route("/tags_fetch",post())
        // .route("/article_new",post())
        // .route("/article_del",post())
        // .route("/articles_getwithtag",post)
        // .route("/comment",post);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}



