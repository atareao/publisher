use sqlx::sqlite::SqlitePool;
use sqlx::{query, Executor};

#[derive(Debug, sqlx::FromRow)]
pub struct Day{
    id: Option<i32>,
    name: String,
}

impl Day{
    /*
    pub fn all() -> Vec<Day>{
    }
    */

    pub await fn new(pool: &SqlitePool, name: &str) -> Self{
        let mut connection = pool.acquire().await?;
        query!(r#"INSERT INTO days (name) VALUES (?1)"#, name);

        let id = query!(r#"INSERT INTO days (name) VALUES (?1)"#, name)
            .execute(&mut connection)
            .await?
            .last_insert_rowid();
        Day{
            id: Some(id),
            name: name.to_string(),
        }
    }
}
