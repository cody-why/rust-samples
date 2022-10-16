// fn main() {
//     println!("Hello, world!");
// }

use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use rocket::Request;

#[macro_use]
extern crate rocket;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<name>")]
fn hello(name: &str) -> String {
    let reply = format!("Welcome, {}!", name);
    reply
}

#[get("/user/<id>")]
fn get_user(id: u64) -> Json<User> {
    Json(User {
        id: id,
        name: format!("user_{}", id),
    })
}

#[post("/user",data="<user>",format="json")]
fn creat_user(user:Json<User>) -> Json<User> {
    Json(User {
        id: user.id,
        name: format!("user_{}", user.name),
    })
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hello, get_user,creat_user])
    .register("/", catchers![not_found])
}
