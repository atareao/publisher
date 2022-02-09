use super::schema::videos;
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Video{
    pub id: Option<i32>,
    pub list_id: i32,
    pub youtube_id: String,
    pub published: bool,
}

#[derive(Debug, Insertable)]
#[table_name = "videos"]
pub struct VideoNew<'a>{
    pub list_id: &'a i32,
    pub youtube_id: &'a str,
    pub published: &'a bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoJson{
    pub list_id: i32,
    pub youtube_id: String,
    pub published: bool,
}
