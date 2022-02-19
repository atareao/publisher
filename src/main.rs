mod day;
mod routes;

use sqlx::sqlite::SqlitePoolOptions;
use actix_web::{web, App, HttpServer};


#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    let db_url = std::env::var("DATABASE_URL").expect("Database not found");
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("pool failed");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(routes::root))
            .route("/day", web::post().to(routes::create_day))
            .route("/day/{day_id}", web::get().to(routes::get_day))
            .route("/day", web::post().to(routes::create_day))
            .route("/days", web::get().to(routes::get_days))
    })
    .bind("localhost:8080")
    .unwrap()
    .run()
    .await
}
