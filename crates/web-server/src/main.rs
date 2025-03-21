use std::net::SocketAddr;

use axum::{routing::get, Extension, Router};
use root::loader;
use tower_livereload::LiveReloadLayer;

mod config;
mod errors;
mod root;
mod static_files;

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let pool = db::create_pool(&config.database_url);

    let app = Router::new()
        .route("/", get(loader))
        .route("/static/*path", get(static_files::static_path))
        .layer(LiveReloadLayer::new())
        .layer(Extension(config))
        .layer(Extension(pool.clone()));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap()
}
