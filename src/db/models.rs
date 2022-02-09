use super::schema::{days, lists, videos, day_list};
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};


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

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct DayList{
    pub id: Option<i32>,
    pub day_id: i32,
    pub list_id: i32,
    pub norder: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "day_list"]
pub struct DayListNew<'a>{
    pub day_id: &'a i32,
    pub list_id: &'a i32,
    pub norder: &'a i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DayListJson{
    pub day_id: i32,
    pub list_id: i32,
    pub norder: i32,
}
