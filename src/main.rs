mod day;
mod routes;

use sqlx::sqlite::SqlitePoolOptions;
use actix_web::{web, App, HttpServer};


#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    let database_url = std::env::var("DATABASE_URL").expect("Database not found");
    //let database_pool = SqlitePool::connect(&database_url).await;
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("pool failed");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(routes::root))
            //.route("/users", web::post().to(routes::create_user))
            .route("/days", web::get().to(routes::get_days))
    })
    .bind("localhost:8080")
    .unwrap()
    .run()
    .await
}
