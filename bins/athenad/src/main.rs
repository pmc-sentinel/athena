use anyhow::Result;
use athena_data::conn::Db;
use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use config::{config, Config};
use tokio::net::TcpListener;

mod config;
mod errors;
mod handlers;

#[derive(Clone)]
pub struct AppState {
    config: Config,
    db: Db,
}

async fn make_router(db: Db) -> Router {
    Router::new()
        .route("/servers", get(handlers::servers::list_servers))
        .route("/servers", post(handlers::servers::new_server))
        .route("/servers/:server_id", get(handlers::servers::get_server))
        .route(
            "/servers/:server_id",
            patch(handlers::servers::update_server),
        )
        .route(
            "/servers/:server_id",
            delete(handlers::servers::delete_server),
        )
        .route(
            "/servers/:server_id/upgrade",
            post(handlers::servers::upgrade_server),
        )
        .route(
            "/servers/:server_id/start",
            post(handlers::servers::start_server),
        )
        .with_state(AppState {
            config: config().await,
            db,
        })
}

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = config().await;

    let db = athena_data::conn::connect(cfg.db_addr, cfg.db_user, cfg.db_password).await?;
    init_tracing();

    let router = make_router(db.clone()).await;

    let lis = TcpListener::bind("127.0.0.1:8080").await?;
    Ok(axum::serve(lis, router).await?)
}

fn init_tracing() {
    tracing_subscriber::fmt().init()
}
