use actix_web::web;
use sqlx::{sqlite::SqlitePool, query, query_as, FromRow, Error};
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Day{
    pub id: i64,
    pub name: String,
}

impl Day{
    pub async fn get_all(pool: web::Data<SqlitePool>) -> Result<Vec<Day>, Error>{
        let days = query_as!(Day, r#"SELECT id, name FROM days"#)
            .fetch_all(pool.get_ref())
            .await?;
        Ok(days)
    }

    pub async fn get_first(pool: web::Data<SqlitePool>) -> Result<Day, Error>{
        let days = query_as!(Day, r#"SELECT id, name FROM days LIMIT 1"#)
            .fetch_one(pool.get_ref())
            .await?;
        Ok(days)
    }

    pub async fn get(pool: web::Data<SqlitePool>, id: i64) -> Result<Day, Error>{
        let day = query_as!(Day, r#"SELECT id, name FROM days WHERE id=$1"#, id)
            .fetch_one(pool.get_ref())
            .await?;
        Ok(day)
    }

    pub async fn new(pool: web::Data<SqlitePool>, name: &str) -> Result<Day, Error>{
        let id = query("INSERT INTO days (name) VALUES (?);")
            .bind(name)
            .execute(pool.get_ref())
            .await?
            .last_insert_rowid();
        Ok(Self::get(pool, id).await?)
    }

    pub async fn delete(pool: web::Data<SqlitePool>, id: i64) -> Result<Day, Error>{
        let day = query_as!(Day, r#"SELECT id, name FROM days WHERE id=$1"#, id)
            .fetch_one(pool.get_ref())
            .await?;
        query("DELETE FROM days WHERE id = ?")
            .bind(id)
            .execute(pool.get_ref())
            .await?;
        Ok(day)
    }

    pub async fn update(pool: web::Data<SqlitePool>, id: i64, name: &str) -> Result<Day, Error>{
        query("UPDATE days SET name=? WHERE id=?")
            .bind(name)
            .bind(id)
            .execute(pool.get_ref())
            .await?;
        Ok(Self::get(pool, id).await?)
    }
}
