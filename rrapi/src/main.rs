#[macro_use] extern crate rocket;

#[derive(Debug)]
struct User {
    uuid: String,
    name: String
}

use rocket::{ http::Status, serde::json::Json };


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/")]
fn hello() -> Result<Json<String>, Status> {
    Ok(Json(String::from("{value:Yes}")))
}

#[get("/<uuid>?<name>")]
fn createUser( uuid: String,  ){

}


#[launch]
fn rocket() -> _ {
    let mut users: Vec<User> = Vec::new();
    users.push(User { uuid: "ok".into(), name: "Jo".into() });
    for user in users.iter() {
        println!("{}", user.name)
    }
    rocket::build()
        .mount("/", routes![index])
        .mount("/sayHi", routes![hello])
}