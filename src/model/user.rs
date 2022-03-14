use diesel::prelude::*;
use rocket::serde::{Serialize, Deserialize};

use crate::schema::*;
use crate::establish_connection;

#[derive(Debug, Serialize, Deserialize, Insertable, Queryable, AsChangeset)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub nick_name: String,
    pub phone: String
}

impl User {
    pub fn create(mut user: User) -> bool {
        user.id = Some(0);

        if let Some(users) = User::list() {
            user.id = Some(users.len() as i32);
        };

        let result = diesel::insert_into(users::table).values(user)
            .execute(&establish_connection());

        result.unwrap() > 0
    }

    pub fn list() -> Option<Vec<User>> {
        match users::table.load::<User>(&establish_connection()) {
            Ok(collection) => Some(collection),
            Err(_) => None
        }
    }

    pub fn get(id: i32) -> Option<User> {
        match users::table.filter(users::id.eq(id))
            .first(&establish_connection()) {
            Ok(user) => Some(user),
            Err(_) => None
        }
    }

    pub fn delete(id: i32) -> bool {
        let result = diesel::delete(users::table
            .filter(users::id.eq(id)))
            .execute(&establish_connection());

        result.unwrap() > 0
    }

    pub fn update(id: i32, user: User) -> bool {
        let result = diesel::update(users::table)
            .filter(users::id.eq(id))
            .set(user)
            .execute(&establish_connection());

        result.unwrap() > 0
    }
}