#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod db;
use db::*;

fn main() {
    println!("Hello, world!");
    println!("Saludos desde el infinito");
}
