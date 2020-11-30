#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::request::Request;

#[get("/")]
fn index() -> &'static str {
    "Hello, from Rust!"
}

#[catch(503)]
fn service_not_available(_req: &Request) -> &'static str {
    "Service is not available. (Is the database up?)"
}

fn main() {
    rocket::ignite()
        .register(catchers![service_not_available])
        .mount("/api", routes![index])
        .launch();
}
