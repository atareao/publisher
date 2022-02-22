use actix_web::{get, post, web, Error, HttpResponse, http::StatusCode};
use anyhow::Result;
use sqlx::SqlitePool;
use crate::day::Day;
use crate::list::List;

#[get("/")]
pub async fn root() -> Result<HttpResponse, Error>{
    Ok(HttpResponse::build(StatusCode::OK).body("Hello world, Rust!"))
}

#[get("/days")]
pub async fn get_days(pool: web::Data<SqlitePool>) -> Result<HttpResponse, Error>{
    Ok(Day::get_all(pool)
       .await
       .map(|some_days| HttpResponse::Ok().json(some_days))
       .map_err(|_| HttpResponse::InternalServerError())?)
}

#[get("/days/{day_id}")]
pub async fn get_day(pool: web::Data<SqlitePool>, web::Path(day_id): web::Path<i64>) -> Result<HttpResponse, Error>{
    Ok(Day::get(pool, day_id)
       .await
       .map(|day| HttpResponse::Ok().json(day))
       .map_err(|_| HttpResponse::NotFound())?)
}

#[post("/days")]
pub async fn create_day(pool: web::Data<SqlitePool>, data: web::Json<Day>) -> Result<HttpResponse, Error>{
    Ok(Day::new(pool, &data.into_inner().name)
       .await
       .map(|day| HttpResponse::Ok().json(day))
       .map_err(|_| HttpResponse::InternalServerError())?)
}

#[get("/lists")]
pub async fn get_lists(pool: web::Data<SqlitePool>) -> Result<HttpResponse, Error>{
    Ok(List::get_all(pool)
       .await
       .map(|some_lists| HttpResponse::Ok().json(some_lists))
       .map_err(|_| HttpResponse::InternalServerError())?)
}
