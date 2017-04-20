use crypto::scrypt::*;
use db::DB;
use diesel;
use diesel::associations::HasTable;
use diesel::prelude::*;
use errors::*;
use models::*;
use rocket_contrib::JSON;
use util::{SCRYPT_PARAMS, sign_jwt, validate_cardnum, validate_email, validate_username};

#[derive(Debug, Deserialize)]
pub struct NewUserRequest {
    email: String,
    realname: Option<String>,
    username: String,
    password: String,
    cardnum: Option<String>,
}   

#[post("/new", data = "<req>")]
pub fn new_user(db: DB, req: JSON<NewUserRequest>) -> Result<String> {
    use schema::users::dsl::*;
 
    let new_user = try!(build_new_user(req.into_inner(), &db));
    let user: User = try!(diesel::insert(&new_user)
        .into(users::table())
        .get_result(db.conn()));
    sign_jwt(&user.auth_token())
}

/// Validates the NewUserRequest and creates a NewUser.
fn build_new_user(req: NewUserRequest, db: &DB) -> Result<NewUser> {
    let scrypt_params = &*SCRYPT_PARAMS;

    // Try validating the email; if the email is invalid, bail out.
    try!(validate_email(&req.email));

    // Try validating the realname. This really just means making the empty
    // string into None; real names are hard.
    let realname = if req.realname == Some("".to_string()) {
        None
    } else {
        req.realname
    };

    // Try validating the username by first checking if it's valid and then
    // checking for duplication.
    try!(validate_username(&req.username));
    let user_count: i64 = try!({
        use schema::users::dsl::*;
        users.filter(username.eq(&req.username).or(email.eq(&req.email)))
            .count()
            .get_result(db.conn())
    });
    if user_count > 0 {
        return Err(ErrorKind::DuplicateUser.into());
    }

    // Compute the password hash.
    let passhash = try!(scrypt_simple(&req.password, &scrypt_params));
 
    // If the card number is present, add it to the account.
    let (cardhash, cardlast) = if let Some(cardnum) = req.cardnum {
        try!(validate_cardnum(&cardnum));
        let cardhash = try!(scrypt_simple(&cardnum, &scrypt_params));
        let cardlast = cardnum.chars()
            .rev()
            .take(4)
            .collect();
        (Some(cardhash), Some(cardlast))
    } else {
        (None, None)
    };

    // Create the NewUser and return it.
    Ok(NewUser {
        email: req.email,
        realname: realname,
        username: req.username,
        passhash: passhash,
        cardhash: cardhash,
        cardlast: cardlast,
    })
}
