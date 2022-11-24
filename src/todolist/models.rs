use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, FromRow};

pub struct AppState {
    pub db: Pool<Postgres>
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct TodoList {
    pub id: i32,
    pub title: String,
    pub date: i32,
}

#[derive(Clone, Deserialize)]
pub struct CreateEntryData {
    pub title: String,
    pub date: i32,
}

#[derive(Clone, Deserialize)]
pub struct UpdateEntryData {
    pub title: String,
 }