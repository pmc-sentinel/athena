use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ModpackSource {
    PresetHtml(String)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Modpack {
    pub id: Thing,
    pub name: String,
    pub description: Option<String>,
    pub source: ModpackSource,
}
