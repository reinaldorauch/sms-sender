use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, Header, EncodingKey};
use super::models::User;

const ONE_DAY: usize = 24 * 60 * 60;

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
  sub: i32,
  user_type: String,
  exp: usize,
}

pub fn create_jwt(user: &User) -> String {
  let claims = Claims {
    sub: user.user_id,
    user_type: String::from(&user.user_type),
    exp: ONE_DAY, // 1 day
  };
  encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref()))
    .expect("Could not encode jwt")
}