use actix_web::http::StatusCode;
use actix_web::{web, Error, HttpResponse, Responder};
use anyhow::Result;
use sqlx::SqlitePool;
use crate::day::Day;

pub async fn root() -> Result<HttpResponse, Error>{
    Ok(HttpResponse::build(StatusCode::OK).body("Hello world, Rust!"))
}

/*
pub async fn list_days(pool: SqlitePool) -> Result<HttpResponse, Error>{
    Ok(Day::all(&pool))
}
*/
pub async fn get_days(pool: web::Data<SqlitePool>) -> Result<HttpResponse, Error>{
    Ok(Day::get_all(&pool)
       .await
       .map(|some_days| HttpResponse::Ok().json(some_days))
       .map_err(|_| HttpResponse::InternalServerError())?)
}


/*
async fn list_days(pool: web::Data<Pool>) -> Result<Vec<Day>, diesel::result::Error>{
    let connection = pool.get().unwrap();
    let result = Day::list(&connection);
    Ok(result)
}
pub async fn get_days(pool: web::Data<Pool>) -> Result<HttpResponse, Error>{
    Ok(list_days(pool)
       .await
       .map(|some_days| HttpResponse::Ok().json(some_days))
       .map_err(|_| HttpResponse::InternalServerError())?)
}
*/
