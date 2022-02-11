use super::schema::lists;
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct List{
    pub id: Option<i32>,
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


