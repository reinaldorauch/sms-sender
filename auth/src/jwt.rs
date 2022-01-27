use rocket::request::{FromRequest, Outcome, Request};
use rocket::http::Status;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, Header, EncodingKey, decode, DecodingKey, Validation, Algorithm, errors::Error};
use super::models::User;

const ONE_DAY: usize = 24 * 60 * 60;

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
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

pub fn validate_jwt<'r>(token: &'r str) -> Result<Claims, Error> {
    match decode::<Claims>(token, &DecodingKey::from_secret("secret".as_ref()), &Validation::new(Algorithm::HS256)) {
        Ok(data) => Ok(data.claims),
        Err(e) => Err(e)
    } 
}


#[derive(Debug)]
pub enum TokenError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Claims {
   type Error  = TokenError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("Authorization") {
            None => Outcome::Failure((Status::BadRequest, TokenError::Missing)),
            Some(key) => {
                let slices: Vec<&'r str> = key.split_whitespace().collect();

                if slices.len() != 2 {
                  return Outcome::Failure((Status::BadRequest, TokenError::Invalid))
                }

                match validate_jwt(slices[1]) {
                  Ok(payload) => Outcome::Success(payload),
                  Err(_) => Outcome::Failure((Status::BadRequest, TokenError::Invalid))
                }
            }
        }
    }
}
