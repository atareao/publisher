use actix_web::http::StatusCode;
use actix_web::{web, Error, HttpResponse};
use anyhow::Result;
use sqlx::SqlitePool;
use crate::day::Day;

pub async fn root() -> Result<HttpResponse, Error>{
    Ok(HttpResponse::build(StatusCode::OK).body("Hello world, Rust!"))
}

pub async fn get_days(pool: web::Data<SqlitePool>) -> Result<HttpResponse, Error>{
    Ok(Day::get_all(pool)
       .await
       .map(|some_days| HttpResponse::Ok().json(some_days))
       .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn get_day(pool: web::Data<SqlitePool>, web::Path(day_id): web::Path<i64>) -> Result<HttpResponse, Error>{
    Ok(Day::get(pool, day_id)
       .await
       .map(|day| HttpResponse::Ok().json(day))
       .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn create_day(pool: web::Data<SqlitePool>, data: web::Json<Day>) -> Result<HttpResponse, Error>{
    Ok(Day::new(pool, &data.into_inner().name)
       .await
       .map(|day| HttpResponse::Ok().json(day))
       .map_err(|_| HttpResponse::InternalServerError())?)
}
