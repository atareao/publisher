//mod day;
mod routes;

use sqlx::sqlite::SqlitePool;
use actix_web::{web, App, HttpServer};



#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    let database_url = std::env::var("DATABASE_URL").expect("NOT FOUND");
    let database_pool = SqlitePool::connect(&database_url).await;
    HttpServer::new(move || {
        App::new()
            //.data(database_pool.clone())
            .route("/", web::get().to(routes::root))
            //.route("/users", web::post().to(routes::create_user))
            //.route("/days", web::get().to(routes::get_days))
    })
    .bind("localhost:8080")
    .unwrap()
    .run()
    .await
}
