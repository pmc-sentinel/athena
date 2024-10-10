use crate::{
    errors::{AppError, Result},
    AppState,
};
use athena_data::server::{get_servers, NewServer};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

pub async fn list_servers(State(state): State<AppState>) -> Result<impl IntoResponse> {
    let servers = get_servers(&state.db).await?;
    Ok(Json(servers))
}

pub async fn get_server(
    State(state): State<AppState>,
    Path(server_id): Path<String>,
) -> Result<impl IntoResponse> {
    let server = athena_data::server::get_server(&state.db, server_id).await?;
    match server {
        Some(srv) => Ok(Json(srv)),
        None => Err(AppError::NotFound),
    }
}

#[derive(Deserialize)]
pub struct NewServerBody {
    pub name: String,
    pub port: i16,
    pub limit_fps: i16,
    pub extra_flags: Vec<String>,
}

pub async fn new_server(
    State(state): State<AppState>,
    Json(body): Json<NewServerBody>,
) -> Result<impl IntoResponse> {
    let server = athena_data::server::create_server(
        &state.db,
        NewServer {
            name: body.name,
            port: body.port,
            limit_fps: body.limit_fps,
            extra_flags: body.extra_flags,
            world: String::from("empty"),
            modpack: None,
        },
    )
    .await?;

    std::fs::create_dir_all(server.install_path())?;
    std::fs::create_dir_all(server.profiles_path())?;

    Ok(Json(server))
}

#[derive(Deserialize)]
pub struct UpdateServerBody {
    pub name: Option<String>,
}

pub async fn update_server(
    State(state): State<AppState>,
    Path(server_id): Path<String>,
    Json(body): Json<UpdateServerBody>,
) -> Result<impl IntoResponse> {
    let server = athena_data::server::get_server(&state.db, server_id).await?;

    match server {
        Some(srv) => {
            let mut updated = srv.clone();

            if let Some(name) = body.name {
                updated.name = name;
            }

            let new = athena_data::server::update_server(&state.db, updated).await?;
            Ok(Json(new))
        }
        None => Err(AppError::NotFound),
    }
}

pub async fn delete_server(
    State(state): State<AppState>,
    Path(server_id): Path<String>,
) -> Result<impl IntoResponse> {
    if athena_data::server::get_server(&state.db, server_id.clone())
        .await?
        .is_none()
    {
        return Err(AppError::NotFound);
    }

    athena_data::server::delete_server(&state.db, server_id.clone()).await?;
    Ok(())
}

pub async fn upgrade_server(
    State(state): State<AppState>,
    Path(server_id): Path<String>,
) -> Result<impl IntoResponse> {
    let server = athena_data::server::get_server(&state.db, server_id).await?;

    match server {
        Some(srv) => {
            std::fs::create_dir_all(srv.install_path())?;
            std::fs::create_dir_all(srv.profiles_path())?;
            tokio::spawn(async move {
                srv.install_update(state.config.steam_username, state.config.steam_password)
                    .await
            });
            Ok(())
        }
        None => Err(AppError::NotFound),
    }
}

pub async fn start_server(
    State(state): State<AppState>,
    Path(server_id): Path<String>,
) -> Result<impl IntoResponse> {
    let server = athena_data::server::get_server(&state.db, server_id).await?;

    match server {
        Some(srv) => {
            std::fs::create_dir_all(srv.install_path())?;
            std::fs::create_dir_all(srv.profiles_path())?;
            tokio::spawn(async move { srv.launch().await });
            Ok(())
        }
        None => Err(AppError::NotFound),
    }
}
