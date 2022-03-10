use actix_web::{get, post, delete, web, Error, HttpResponse, http::StatusCode};
use anyhow::Result;
use sqlx::SqlitePool;
use crate::day::{Day, NewDay};
use crate::list::List;
use crate::video::Video;

#[get("/status")]
pub async fn root() -> Result<HttpResponse, Error>{
    Ok(HttpResponse::build(StatusCode::OK).body("Up and running"))
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
#[delete("/days/{day_id}")]
pub async fn delete_day(pool: web::Data<SqlitePool>, web::Path(day_id): web::Path<i64>) -> Result<HttpResponse, Error>{
    Ok(Day::delete(pool, day_id)
       .await
       .map(|day| HttpResponse::Ok().json(day))
       .map_err(|_| HttpResponse::NotFound())?)
}

#[post("/days")]
pub async fn create_day(pool: web::Data<SqlitePool>, data: web::Json<NewDay>) -> Result<HttpResponse, Error>{
    Ok(Day::new(pool, &data.into_inner().name)
       .await
       .map(|day| HttpResponse::Ok().json(day))
       .map_err(|_| HttpResponse::InternalServerError())?)
}

#[get("/videos")]
pub async fn get_videos(pool: web::Data<SqlitePool>) -> Result<HttpResponse, Error>{
    Ok(Video::get_all(pool)
       .await
       .map(|some_videos| HttpResponse::Ok().json(some_videos))
       .map_err(|_| HttpResponse::InternalServerError())?)
}

#[get("/lists")]
pub async fn get_lists(pool: web::Data<SqlitePool>) -> Result<HttpResponse, Error>{
    Ok(List::get_all(pool)
       .await
       .map(|some_lists| HttpResponse::Ok().json(some_lists))
       .map_err(|_| HttpResponse::InternalServerError())?)
}
#[get("/lists/{list_id}")]
pub async fn get_list(pool: web::Data<SqlitePool>, web::Path(list_id): web::Path<String>) -> Result<HttpResponse, Error>{
    Ok(List::get(pool, &list_id)
       .await
       .map(|list| HttpResponse::Ok().json(list))
       .map_err(|_| HttpResponse::NotFound())?)
}
