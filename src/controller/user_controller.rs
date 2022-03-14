use rocket::serde::json::{ Json, Value };
use rocket::serde::json::serde_json::json;

use crate::model::user::{User};

#[post("/", format = "json", data = "<user>")]
pub fn create_user(user: Json<User>) -> Value {
    match User::create(user.into_inner()) {
        true => json!({ "status": "created" }),
        false => json!({ "status": "failed" })
    }
}

#[get("/")]
pub fn get_users() -> Value {
    if let Some(users) = User::list() {
        return json!({ "status": "found", "users": users });
    }

    else {
        json!({ "status": "failed" })
    }

}

#[get("/<id>")]
pub fn get_user_by_id(id: i32) -> Value {
    if let Some(user) = User::get(id) {
        return json!({ "status": "found", "user": user });
    }

    else {
        json!({ "status": "failed" })
    }
}

#[delete("/<id>")]
pub fn delete_user_by_id(id: i32) -> Value {
    match User::delete(id) {
        true => json!({ "status": "deleted" }),
        false => json!({ "status": "failed" })
    }
}

#[put("/<id>", format = "json", data = "<user>")]
pub fn update_user_by_id(id: i32, user: Json<User>) -> Value {
    match User::update(id, user.into_inner()) {
        true => json!({ "status": "updated" }),
        false => json!({ "status": "failed" })
    }
}