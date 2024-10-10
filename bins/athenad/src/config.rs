use std::env;

use tokio::sync::OnceCell;

#[derive(Clone, Debug)]
pub struct Config {
    pub steam_username: String,
    pub steam_password: String,
    pub db_addr: String,
    pub db_user: String,
    pub db_password: String,
}

static CONFIG: OnceCell<Config> = OnceCell::const_new();

async fn init_config() -> Config {
    Config {
        steam_username: env::var("STEAM_USERNAME").expect("Must set STEAM_USERNAME."),
        steam_password: env::var("STEAM_PASSWORD").expect("Must set STEAM_PASSWORD."),
        db_addr: env::var("DB_ADDR").expect("Must set DB_ADDR."),
        db_user: env::var("DB_USER").expect("Must set DB_USER."),
        db_password: env::var("DB_PASSWORD").expect("Must set DB_PASSWORD."),
    }
}

pub async fn config() -> Config {
    CONFIG.get_or_init(init_config).await.clone()
}
