#[macro_use] extern crate rocket;

#[get("/")]
fn ping() -> &'static str {
  "Pong"
}

#[get("/healthcheck")]
fn healthcheck() -> &'static str {
  "Service OK!"
}

#[launch]
fn rocket() -> _ {
   rocket::build().mount("/", routes![ping, healthcheck])
}
