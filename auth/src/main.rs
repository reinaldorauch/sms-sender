// Using macros from rocket crate
#[macro_use] extern crate rocket;

// Telling cargo that the tests are in tests module
#[cfg(test)] mod tests;

use rocket::{Rocket, Build, form::Form};
use auth_service;

// This route tells a cluster manager that this service is well and can receive
// requests
#[get("/healthcheck")]
fn healthcheck() -> &'static str {
    "Success"
}

#[derive(FromForm, Clone)]
struct LoginRequest {
  username: String,
  password: String
}


// main routes
#[post("/login", data="<login>")]
fn login(login: Form<LoginRequest>) -> String {
    auth_service::do_login(&login.username, &login.password).unwrap()
}

#[derive(FromForm, Clone)]
struct RegisterRequest {
  username: String,
  email: String,
  password: String,
  password_confirm: String
}


#[post("/register", data = "<req>")]
fn register(req: Form<RegisterRequest>) -> &'static str {
    if req.password != req.password_confirm {
        return "err"
    }
    auth_service::register(&req.username, &req.email, &req.password).unwrap();
    "Register Success"
}

#[post("/logout")]
fn logout() -> &'static str {
  "Success logout"
}

// Error routes
#[catch(404)]
fn not_found() -> &'static str {
    "Not found"
}

fn rocket() -> Rocket<Build> {
    // This is the root of the api
    rocket::build()
        .mount("/", routes![healthcheck, login, register, logout])
        .register("/", catchers![not_found])
}

#[rocket::main]
async fn main() {
    if let Err(e) = rocket().launch().await {
        println!("Whoops! Rocket didn't launch!");
        // We drop the error to get a Rocket-formatted panic.
        drop(e);
    };
}

