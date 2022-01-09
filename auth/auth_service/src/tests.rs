#[test]
pub fn test_do_login() {
  let username = String::from("ReinaldoRauch");
  let password = String::from("oloco");
  assert_eq!(
    super::do_login(
      &username,
      &password
    ), 
    String::from("ReinaldoRauch: oloco")
  )
}