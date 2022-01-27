use rocket::local::blocking::Client;
use rocket::http::Status;

#[test]
fn test_healthcheck() {
  let client = Client::tracked(super::rocket()).unwrap();

  let response = client.get("/healthcheck").dispatch();

  assert_eq!(response.status(), Status::Ok);
  assert_eq!(response.into_string().unwrap(), super::healthcheck())
}

#[test]
fn test_login() {
  let client = Client::tracked(super::rocket()).unwrap();

  let response = client.post("/login").dispatch();

  assert_eq!(response.status(), Status::Ok);
  assert_eq!(response.into_string().unwrap(), super::login())
}

#[test]
fn test_register() {
  let client = Client::tracked(super::rocket()).unwrap();

  let response = client.post("/register").dispatch();

  assert_eq!(response.status(), Status::Ok);
  assert_eq!(response.into_string().unwrap(), super::register())
}

#[test]
fn test_logout() {
  let client = Client::tracked(super::rocket()).unwrap();

  let response = client.post("/logout").dispatch();

  assert_eq!(response.status(), Status::Ok);
  assert_eq!(response.into_string().unwrap(), super::logout())
}

#[test]
pub fn test_do_login() {
  let username = String::from("ReinaldoRauch");
  let password = String::from("oloco");
  assert_eq!(
    super::auth_service::do_login(
      &username,
      &password
    ), 
    String::from("ReinaldoRauch: oloco")
  )
}