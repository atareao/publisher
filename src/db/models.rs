use super::schema::{days, lists};
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};


#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Day{
    pub id: i32,
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

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct List{
    pub id: i32,
    pub youtube_id: String,
    pub name: String,
    pub reverse: bool,
}

#[derive(Debug, Insertable)]
#[table_name = "lists"]
pub struct ListNew<'a>{
    pub youtube_id: &'a str,
    pub name: &'a str,
    pub reverse: &'a bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListJson{
    pub name: String,
    pub youtube_id: String,
    pub reverse: bool,
}
