pub mod day;
pub mod video;
pub mod day_list;
pub mod list;
pub mod schema;

use diesel::prelude::*;
use day::*;
use dotenv::dotenv;
use std::env;

embed_migrations!();

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecto to {}", database_url))
      //.unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_days() -> Vec<Day> {
    let connection = establish_connection();
    Day::list(&connection)
}
