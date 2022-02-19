use sqlx::sqlite::SqlitePool;
use sqlx::{query_as, FromRow, Error};
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Day{
    pub id: i64,
    pub name: String,
}

impl Day{
    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Day>, Error>{
        let days = query_as!(Day, r#"SELECT id, name FROM days"#)
            .fetch_all(pool)
            .await?;
        Ok(days)
    }

    pub async fn get_first(pool: &SqlitePool) -> Result<Day, Error>{
        let days = query_as!(Day, r#"SELECT id, name FROM days LIMIT 1"#)
            .fetch_one(pool)
            .await?;
        Ok(days)
    }

    pub async fn new(pool: &SqlitePool, name: &str) -> Result<Day, Error>{
        let aday = query_as!(Day,
                           r#"INSERT INTO days (name) VALUES ($1) returning id, name"#, name)
            .fetch_one(pool)
            .await?;
        Ok(aday)
    }
}
