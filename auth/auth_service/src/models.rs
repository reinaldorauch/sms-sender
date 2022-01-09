use diesel::pg::PgConnection;
use super::schema::user;

#[derive(Queryable)]
pub struct User {
  pub user_id: i32,
  pub email: String,
  pub username: String,
  pub password_hash: String,
  pub user_type: String,
}

impl User {
  pub fn find_by_username(conn: &PgConnection, username_query: &String) -> User {
    use diesel::prelude::*;
    use self::user::dsl::*;

    user.filter(username.eq(username_query))
      .first::<User>(conn)
      .expect("Error finding user")
  }

  pub fn verify_password(&self, password: &String) -> bool {
    use argonautica::Verifier;

    Verifier::default()
      .with_password(password)
      .with_hash(&self.password_hash)
      .with_secret_key("")
      .verify()
      .unwrap()
  }
}

#[derive(Insertable)]
#[table_name="user"]
pub struct NewUser {
  pub username: String,
  pub email: String,
  pub password_hash: String,
  pub user_type: String
}

impl NewUser {
  pub fn create(username: &String, email: &String, password: &String, user_type: &String) -> NewUser {
    NewUser {
      username: String::from(username),
      email: String::from(email),
      password_hash: hash_password(password),
      user_type: String::from(user_type)
    }
  }
}

fn hash_password(password: &String) -> String {
  use argonautica::Hasher;
  use argonautica::config::Variant;

  Hasher::default()
    .configure_iterations(192)
    .configure_lanes(8)
    .configure_memory_size(4096)
    .configure_variant(Variant::Argon2d)
    .with_secret_key("")
    .with_password(password)
    .hash()
    .unwrap()
}