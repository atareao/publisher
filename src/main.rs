mod day;
mod list;
mod video;
mod routes;

use sqlx::sqlite::SqlitePoolOptions;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;


#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("Database not found");
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("pool failed");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(routes::create_day)
            .service(routes::get_days)
            .service(routes::root)
            .service(routes::get_day)
            .service(routes::get_lists)
            .service(routes::get_videos)
    })
    .bind("localhost:8080")
    .unwrap()
    .run()
    .await
}
