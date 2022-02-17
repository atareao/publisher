use diesel::r2d2::Error;
use r2d2::ManageConnection;
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, sql_query};
use diesel::prelude::*;
use diesel::dsl::max;
use super::schema::days;
use super::schema::days::dsl::days as day_dsl;


#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
pub struct Day{
    pub id: Option<i32>,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DayJson{
    pub name: String,
}

impl Day{
    pub fn new(name: &str, connection: &SqliteConnection) -> Option<Self>{
        use super::schema::days::dsl::id;
        let new_day = Day{
            id: None,
            name: name.to_string(),
        };
        diesel::insert_into(days::d)
            .values(&new_day)
            .execute(connection)
            .expect("Error creating new day");

        let sql = "SELECT id, name FROM days WHERE id=(SELECT MAX(id) FROM days)";
        diesel::insert_into(da)
            /*
        diesel::select(sql).execute(connection).unwrap();
        match day_dsl.select(max(id)).first(connection){
            Ok(aday) => Some(day),
            Error(_) => None,
        }
        */
    }
    pub fn by_id(id: i32, connection: &SqliteConnection) -> Option<Self>{
        if let Ok(record) = day_dsl.find(id).get_result::<Day>(connection){
            Some(record)
    }else{
        None
        }
    }
    pub fn list(connection: &SqliteConnection) -> Vec<Self>{
        day_dsl.load::<Day>(connection).expect("Error loading users")
    }

}

#[cfg(test)]
mod test_day;
