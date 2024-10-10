use anyhow::Result;
use include_dir::{Dir, include_dir};
use surrealdb::{engine::remote::ws::{Ws, Client}, opt::auth::Root, Surreal};
use surrealdb_migrations::MigrationRunner;
use thiserror::Error;

static DB_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/db");

#[derive(Error, Debug)]
pub enum ConnErr {
    #[error("Migrations error: {0}")]
    MigrateErr(#[from] eyre::Report),

    #[error("SurrealDB error: {0}")]
    SurrealErr(#[from] surrealdb::Error),
}

pub type Db = Surreal<Client>;

async fn migrate(db: &Db) -> Result<(), ConnErr> {
    MigrationRunner::new(db)
        .load_files(&DB_DIR)
        .up()
        .await?;

    Ok(())
}

pub async fn connect(addr: String, user: String, password: String) -> Result<Db> {
    let db = Surreal::new::<Ws>(addr).await?;

    db.signin(Root {
        username: &user,
        password: &password,
    }).await?;

    db.use_ns("athena").use_db("athena").await?;

    migrate(&db).await?;

    Ok(db)
}
