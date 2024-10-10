use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs::File, process::Stdio, io::prelude::*};
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};

use crate::modpack::Modpack;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Server {
    pub id: Thing,
    pub name: String,
    pub port: i16,
    pub limit_fps: i16,
    pub extra_flags: Vec<String>,
    pub world: String,
    pub modpack: Option<Modpack>,
}

impl Server {
    pub fn install_path(&self) -> String {
        format!("/var/lib/athena/servers/{}", self.id)
    }

    pub fn profiles_path(&self) -> String {
        format!("/var/lib/athena/profiles/{}", self.id)
    }

    pub fn logs_path(&self) -> String {
        format!("/var/log/athena/{}", self.id)
    }

    pub fn launch_args(&self) -> Vec<String> {
        let mut args = vec![
            String::from("-name=main"),
            format!("-port={}", self.port),
            format!("-limitFPS={}", self.limit_fps),
            format!("-world={}", self.world),
            format!("-profiles={}", self.profiles_path()),
        ];

        for arg in self.extra_flags.clone() {
            args.push(arg);
        }

        args
    }

    pub async fn install_update(&self, username: String, password: String) {
        std::fs::create_dir_all(self.logs_path()).expect("Couldn't create logs directory");
        let mut log_file = File::create(format!("{}/steamcmd.log", self.logs_path())).expect("Couldn't create logs file");

        let mut cmd = Command::new("/home/hayden/Steam/steamcmd.sh");
        cmd.arg("+force_install_dir");
        cmd.arg(self.install_path());
        cmd.arg("+login");
        cmd.arg(username);
        cmd.arg(password);
        cmd.arg("+app_update");
        cmd.arg("233780");
        cmd.arg("validate");
        cmd.arg("+quit");
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let mut child = cmd.spawn().expect("Failed to start running steamcmd");
        let stdout = child.stdout.take().expect("Failed to take child");
        let mut reader = BufReader::new(stdout).lines();

        tokio::spawn(async move {
            let status = child.wait().await;
            if let Err(s) = status {
                tracing::error!("Error: {}", s);
            }
        });

        while let Some(line) = reader.next_line().await.expect("failed to read logs") {
            log_file.write_all(format!("{}\n",line).as_bytes()).expect("Couldn't write lines to log");
        }
    }

    pub async fn launch(&self) {
        let mut log_file = File::create(format!("{}/server.log", self.logs_path())).expect("Couldn't create logs file");
        let mut cmd = Command::new("./arma3server_x64");
        cmd.current_dir(self.install_path());
        cmd.args(self.launch_args());

        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let mut child = cmd.spawn().expect("Failed to start running steamcmd");
        let stdout = child.stdout.take().expect("Failed to take child");
        let mut reader = BufReader::new(stdout).lines();

        tokio::spawn(async move {
            let status = child.wait().await;
            if let Err(s) = status {
                tracing::error!("Error: {}", s);
            }
        });

        while let Some(line) = reader.next_line().await.expect("failed to read logs") {
            log_file.write_all(format!("{}\n",line).as_bytes()).expect("Couldn't write lines to log");
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewServer {
    pub name: String,
    pub port: i16,
    pub limit_fps: i16,
    pub extra_flags: Vec<String>,
    pub world: String,
    pub modpack: Option<Modpack>,
}

pub async fn get_servers(db: &Surreal<Client>) -> Result<Vec<Server>> {
    let servers: Vec<Server> = db.select("server").await?;
    Ok(servers)
}

pub async fn get_server(db: &Surreal<Client>, id: String) -> Result<Option<Server>> {
    let server: Option<Server> = db.select(("server", id)).await?;
    Ok(server)
}

pub async fn create_server(db: &Surreal<Client>, new_data: NewServer) -> Result<Server> {
    let res: Option<Server> = db.create("server").content(new_data).await?;
    Ok(res.unwrap())
}

pub async fn update_server(db: &Surreal<Client>, new_data: Server) -> Result<Server> {
    let res: Option<Server> = db
        .update(("server", new_data.id.id.to_string()))
        .merge(new_data)
        .await?;
    Ok(res.unwrap())
}

pub async fn delete_server(db: &Surreal<Client>, id: String) -> Result<()> {
    let _: Option<Server> = db.delete(("server", id)).await?;
    Ok(())
}
