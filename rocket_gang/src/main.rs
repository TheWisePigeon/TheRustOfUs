#[macro_use]
extern crate rocket;

use rocket::{ Build, Rocket };

#[get("/")]
fn index() -> &'static str  {
    "Hello Rocket Gang"
}

#[launch]
fn rocket() -> Rocket<Build>{
    rocket::build().mount("/", routes![index])
}