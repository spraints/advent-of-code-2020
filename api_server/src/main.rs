#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

extern crate rocket_contrib;
extern crate serde;

use rocket::request::Request;
use rocket_contrib::json::Json;

mod day1;

#[get("/")]
fn index() -> &'static str {
    "Hello, from Rust!"
}

#[post("/day1", format = "json", data = "<input>")]
fn do_day1(input: Json<day1::Input>) -> Json<day1::Output> {
    Json(day1::solve(input.into_inner()))
}

#[catch(503)]
fn service_not_available(_req: &Request) -> &'static str {
    "Service is not available. (Is the database up?)"
}

fn main() {
    rocket::ignite()
        .register(catchers![service_not_available])
        .mount("/api", routes![index, do_day1])
        .launch();
}
