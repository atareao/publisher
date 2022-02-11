use super::schema::day_list;
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};

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
