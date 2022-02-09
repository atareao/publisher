use crate::day::*;
use crate::Pool;

use actix_web::http::StatusCode;
use actix_web::{web, Error, HttpResponse};
use anyhow::Result;

pub async fn root() -> Result<HttpResponse, Error>{
    Ok(HttpResponse::build(StatusCode::OK).body("Hello world, Rust!"))
}

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
