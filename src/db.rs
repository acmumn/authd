use diesel::pg::PgConnection;
use errors::*;
use r2d2::{Config, GetTimeout, Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;
use rocket::Outcome::{Success, Failure};
use rocket::Request;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use std::env;

fn connect() -> Result<Pool<ConnectionManager<PgConnection>>> {
    let url = try!(env::var("DATABASE_URL"));
    let config = Config::default();
    let manager = ConnectionManager::<PgConnection>::new(url);
    let pool = try!(Pool::new(config, manager));
    Ok(pool)
}

lazy_static! {
     static ref DB_POOL: Pool<ConnectionManager<PgConnection>> = {
         connect().expect("Couldn't connect to the database")
     };
}

pub struct DB {
    conn: PooledConnection<ConnectionManager<PgConnection>>,
}

impl DB {
    pub fn conn(&self) -> &PgConnection {
        &*self.conn
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for DB {
    type Error = GetTimeout;
    fn from_request(_: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match DB_POOL.get() {
            Ok(conn) => Success(DB {
                conn
            }),
            Err(e) => Failure((Status::InternalServerError, e)),
        }
    }
}
