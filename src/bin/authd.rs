//! The application is configured via environment variables. The following are
//! currently defined:
//!
//!  - `DATABASE_URL` -- The URL of the PostgreSQL database to be connected to.
//!  - `ROCKET_ENV` -- `development`, `staging`, or `production`.
//!  - `SCRYPT_N` -- The `log2` of the _n_ constant to the scrypt algorithm.
//!  - `SCRYPT_R` -- The _r_ constant to the scrypt algorithm.
//!  - `SCRYPT_P` -- The _p_ constant to the scrypt algorithm.
//!
//! `DATABASE_URL` and `JWT_SECRET` must be defined. The scrypt constants
//! default to 12, 8, and 1, respectively, while `ROCKET_ENV` defaults to
//! `development`.

#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate error_chain;
extern crate jwt;
extern crate rocket;

#[macro_use]
extern crate snack_authd;
use snack_authd::*;
use snack_authd::errors::*;

quick_main!(|| -> Result<()> {
    // Load environment variables from the .env file.
    try!(dotenv::dotenv());

    require_env_vars![
        DATABASE_URL,
        JWT_SECRET,
    ];

    // Start the server.
    rocket::ignite().mount("/", routes![
        routes::auth,
        routes::new_user,
    ]).catch(errors![
    ]).launch();
    Ok(())
});
