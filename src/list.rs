use actix_web::web;
use sqlx::{sqlite::SqlitePool, query, query_as, FromRow, Error};
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct List{
    pub youtube_id: String,
    pub name: String,
    pub reverse: bool,
}

impl List{
    pub async fn get_all(pool: web::Data<SqlitePool>) -> Result<Vec<List>, Error>{
        let lists = query_as!(List, r#"SELECT youtube_id, name, reverse FROM lists"#)
            .fetch_all(pool.get_ref())
            .await?;
        Ok(lists)
    }

    pub async fn get_first(pool: web::Data<SqlitePool>) -> Result<List, Error>{
        let lists = query_as!(List, r#"SELECT youtube_id, name, reverse FROM lists LIMIT 1"#)
            .fetch_one(pool.get_ref())
            .await?;
        Ok(lists)
    }

    pub async fn get(pool: web::Data<SqlitePool>, youtube_id: &str) -> Result<List, Error>{
        let list = query_as!(List, r#"SELECT youtube_id, name, reverse FROM lists WHERE youtube_id=$1"#, youtube_id)
            .fetch_one(pool.get_ref())
            .await?;
        Ok(list)
    }

    pub async fn new(pool: web::Data<SqlitePool>, youtube_id: &str, name: &str, reverse: bool) -> Result<List, Error>{
        query("INSERT INTO lists (youtube_id, name, reverse) VALUES (?, ?, ?);")
            .bind(youtube_id)
            .bind(name)
            .bind(reverse)
            .execute(pool.get_ref())
            .await?;
        Ok(Self::get(pool, youtube_id).await?)
    }

    pub async fn delete(pool: web::Data<SqlitePool>, youtube_id: &str) -> Result<List, Error>{
        let list = query_as!(List, r#"SELECT youtube_id, name, reverse FROM lists WHERE youtube_id=$1"#, youtube_id)
            .fetch_one(pool.get_ref())
            .await?;
        query("DELETE FROM lists WHERE youtube_id = ?")
            .bind(youtube_id)
            .execute(pool.get_ref())
            .await?;
        Ok(list)
    }

    pub async fn update(pool: web::Data<SqlitePool>, youtube_id: &str, name: &str, reverse: bool) -> Result<List, Error>{
        query("UPDATE days SET name=?, reverse=? WHERE youtube_id=?")
            .bind(name)
            .bind(reverse)
            .bind(youtube_id)
            .execute(pool.get_ref())
            .await?;
        Ok(Self::get(pool, youtube_id).await?)
    }
}
