#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;
extern crate core;

pub mod controller;
pub mod model;
pub mod schema;

use crate::controller::user_controller;
use diesel::prelude::*;

#[rocket::main]
async fn main() {
    rocket::build().mount("/user", routes![
        user_controller::create_user,
        user_controller::get_user_by_id,
        user_controller::get_users,
        user_controller::delete_user_by_id,
        user_controller::update_user_by_id
    ])
        .launch()
        .await;
}

pub fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish("memory.sqlite").unwrap_or_else(
        |_| panic!("Error trying to connect!"))
}
