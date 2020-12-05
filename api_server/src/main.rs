#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

extern crate rocket_contrib;
extern crate serde;

use rocket::request::Request;
use rocket::response::status::BadRequest;
use rocket_contrib::json::Json;
use serde::Serialize;

#[get("/")]
fn index() -> &'static str {
    "Hello, from Rust!"
}

#[derive(Serialize, Debug)]
struct SolutionError {
    error: String,
}

type Solution<T> = Result<Json<T>, BadRequest<Json<SolutionError>>>;

fn resp<T>(solution: Result<T, String>) -> Solution<T> {
    match solution {
        Ok(x) => Ok(Json(x)),
        Err(error) => Err(BadRequest(Some(Json(SolutionError { error })))),
    }
}

#[post("/day1", format = "json", data = "<input>")]
fn do_day1(input: Json<api_server::day1::Input>) -> Solution<api_server::day1::Output> {
    resp(api_server::day1::solve(input.into_inner()))
}

#[post("/day2", format = "json", data = "<input>")]
fn do_day2(input: Json<api_server::day2::Input>) -> Solution<api_server::day2::Output> {
    resp(api_server::day2::solve(input.into_inner()))
}

#[post("/day3", format = "json", data = "<input>")]
fn do_day3(input: Json<api_server::day3::Input>) -> Solution<api_server::day3::Output> {
    resp(api_server::day3::solve(input.into_inner()))
}

#[post("/day4", format = "json", data = "<input>")]
fn do_day4(input: Json<api_server::day4::Input>) -> Solution<api_server::day4::Output> {
    resp(api_server::day4::solve(input.into_inner()))
}

#[post("/day5", format = "json", data = "<input>")]
fn do_day5(input: Json<api_server::day5::Input>) -> Solution<api_server::day5::Output> {
    resp(api_server::day5::solve(input.into_inner()))
}

#[catch(503)]
fn service_not_available(_req: &Request) -> &'static str {
    "Service is not available. (Is the database up?)"
}

fn main() {
    rocket::ignite()
        .register(catchers![service_not_available])
        .mount(
            "/api",
            routes![index, do_day1, do_day2, do_day3, do_day4, do_day5],
        )
        .launch();
}
