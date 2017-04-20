use crypto::scrypt::scrypt_check;
use db::DB;
use diesel::prelude::*;
use errors::*;
use rocket_contrib::JSON;
use models::User;
use util::sign_jwt;

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    username: String,
    password: String,
}   

#[post("/auth", data = "<req>")]
pub fn auth(db: DB, req: JSON<AuthRequest>) -> Result<String> {
    let users: Vec<User> = try!({
        use schema::users::dsl::*;
        users.filter(username.eq(&req.username))
            .limit(1)
            .load(db.conn())
    });
    if users.len() == 0 {
        bail!("No user")
    } else if users.len() > 1 {
        bail!("Too many users")
    } else {
        match scrypt_check(&req.password, &users[0].passhash) {
            Ok(true) => sign_jwt(&users[0].auth_token()),
            Ok(false) => bail!("bad passwd"),
            Err(err) => bail!("{}", err),
        }
    }
}

