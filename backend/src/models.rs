use serde::{Deserialize, Serialize};
use chrono::{Utc, DateTime};
use chrono::serde::ts_seconds;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Timer {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Record {
    pub id: i32,
    pub timer_id: i32,
    #[serde(with = "ts_seconds")]
    pub start_at: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub end_at: DateTime<Utc>,
    pub duration: i32
}