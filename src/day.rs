use sqlx::sqlite::SqlitePool;
use sqlx::{query, Error};

#[derive(Debug, sqlx::FromRow)]
pub struct Day{
    pub id: Option<i64>,
    pub name: String,
}

impl Day{
    pub async fn all(pool: &SqlitePool) -> Result<Vec<Day>, Error>{
        let mut connection = pool.acquire().await?;
        let rdays = query!("SELECT * FROM days")
            .fetch_all(&mut connection)
            .await?;
        let mut days: Vec<Day> = Vec::new();
        for aday in rdays {
            let day = Day{id: Some(aday.id), name: aday.name};
            days.push(day);
        }
        Ok(days)
    }

    pub async fn new(pool: &SqlitePool, name: &str) -> Result<Day, Error>{
        let mut connection = pool.acquire().await?;

        let id = query!("INSERT INTO days (name) VALUES (?)", name)
            .bind(name)
            .execute(&mut connection)
            .await?
            .last_insert_rowid();
        Ok(Day{
            id: Some(id),
            name: name.to_string(),
        })
    }
}
