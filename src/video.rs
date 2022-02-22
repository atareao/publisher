use actix_web::web;
use sqlx::{sqlite::SqlitePool, query, query_as, FromRow, Error};
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video{
    pub youtube_id: String,
    pub youtube_list_id: String,
    pub published: bool,
}

impl Video{
    pub async fn get_all(pool: web::Data<SqlitePool>) -> Result<Vec<Video>, Error>{
        let videos = query_as!(Video, r#"SELECT youtube_id, youtube_list_id, published FROM videos"#)
            .fetch_all(pool.get_ref())
            .await?;
        Ok(videos)
    }

    pub async fn get_first(pool: web::Data<SqlitePool>) -> Result<Video, Error>{
        let videos = query_as!(Video, r#"SELECT youtube_id, youtube_list_id, published FROM videos LIMIT 1"#)
            .fetch_one(pool.get_ref())
            .await?;
        Ok(videos)
    }

    pub async fn get(pool: web::Data<SqlitePool>, youtube_id: &str) -> Result<Video, Error>{
        let video = query_as!(Video, r#"SELECT youtube_id, youtube_list_id, published FROM videos WHERE youtube_id=$1"#, youtube_id)
            .fetch_one(pool.get_ref())
            .await?;
        Ok(video)
    }

    pub async fn new(pool: web::Data<SqlitePool>, youtube_id: &str, youtube_list_id: &str, published: bool) -> Result<Video, Error>{
        query("INSERT INTO videos (youtube_id, youtube_list_id, published) VALUES (?, ?, ?);")
            .bind(youtube_id)
            .bind(youtube_list_id)
            .bind(published)
            .execute(pool.get_ref())
            .await?;
        Ok(Self::get(pool, youtube_id).await?)
    }

    pub async fn delete(pool: web::Data<SqlitePool>, youtube_id: &str) -> Result<Video, Error>{
        let video = query_as!(Video, r#"SELECT youtube_id, youtube_list_id, published FROM videos WHERE youtube_id=$1"#, youtube_id)
            .fetch_one(pool.get_ref())
            .await?;
        query("DELETE FROM videos WHERE youtube_id = ?")
            .bind(youtube_id)
            .execute(pool.get_ref())
            .await?;
        Ok(video)
    }

    pub async fn update(pool: web::Data<SqlitePool>, youtube_id: &str, youtube_list_id: &str, published: bool) -> Result<Video, Error>{
        query("UPDATE videos SET youtube_list_id=?, published=? WHERE youtube_id=?")
            .bind(youtube_list_id)
            .bind(published)
            .bind(youtube_id)
            .execute(pool.get_ref())
            .await?;
        Ok(Self::get(pool, youtube_id).await?)
    }
}
