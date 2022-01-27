use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


pub fn do_login(username: &String, password: &String) -> Result<String, diesel::result::Error> {
    // Checks the database for the user, if succeed, issues a jwt token

    let conn = establish_connection();

    conn.build_transaction()
        .read_only()
        .run(|| {
            use super::models::User;

            let user = User::find_by_username(&conn, username);
            if user.verify_password(password) {
                Ok(super::jwt::create_jwt(&user))
            } else {
                Err(diesel::result::Error::NotFound)
            }
        })
}

pub fn register(username: &String, email: &String, password: &String) -> Result<String, String> {
    use super::models::{NewUser, User};
    use super::schema::user;
    
    let conn = establish_connection();

    let new_user = NewUser::create(username, email, password);
    let insert_result = diesel::insert_into(user::table)
        .values(&new_user)
        .get_result::<User>(&conn);

    match insert_result {
        Ok(user) => Ok(format!("User {} registered with success", user.username)),
        Err(err) => Err(format!("Could not insert user: {}", err))
    }
}
