use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};
use diesel::prelude::*;
use super::schema::days;
use super::schema::days::dsl::days as day_dsl;


#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Day{
    pub id: Option<i32>,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[table_name = "days"]
pub struct DayNew<'a>{
    pub name: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DayJson{
    pub name: String,
}

impl Day{
    pub fn list(connection: &SqliteConnection) -> Vec<Self>{
        day_dsl.load::<Day>(connection).expect("Error loading users")
    }
}
