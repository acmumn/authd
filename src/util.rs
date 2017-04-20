use crypto::scrypt::ScryptParams;
use crypto::sha2::Sha256;
use errors::*;
use jwt::{Component, Token};
use publicsuffix::{List, LIST_URL};
use regex::Regex;
use std::env;
use std::str::FromStr;
use std::sync::Mutex;

lazy_static! {
    pub static ref SCRYPT_PARAMS: ScryptParams = ScryptParams::new(
        parse_env("SCRYPT_N", 12),
        parse_env("SCRYPT_R",  8),
        parse_env("SCRYPT_P",  1),
    );
}

fn parse_env<K: AsRef<::std::ffi::OsStr>, T: FromStr>(name: K, otherwise: T) -> T {
    if let Ok(s) = env::var(name) {
        FromStr::from_str(&s).unwrap_or(otherwise)
    } else {
        otherwise
    }
}

pub fn sign_jwt<H, C>(tok: &Token<H, C>) -> Result<String>
    where H: Component, C: Component
{
    lazy_static! {
        static ref JWT_SECRET: Vec<u8> = env::var("JWT_SECRET")
            .expect("JWT_SECRET not defined")
            .bytes()
            .collect();
    }
    tok.signed(&*JWT_SECRET, Sha256::new())
        .map_err(|e| ErrorKind::Jwt(e).into())
}

pub fn validate_email(email: &str) -> Result<()> {
    lazy_static! {
        static ref PUBLIC_SUFFIX_LIST: Mutex<List> = Mutex::new(List::from_url(LIST_URL)
            .expect("Cannot fetch public suffix list"));
    }
    try!(PUBLIC_SUFFIX_LIST.lock().unwrap().parse_email(email));
    Ok(())
}

pub fn validate_cardnum(cardnum: &str) -> Result<()> {
    lazy_static! {
        static ref CARDNUM_REGEX: Regex = Regex::new("^[0-9]{17}$").unwrap();
    }
    if CARDNUM_REGEX.is_match(cardnum) {
        Ok(())
    } else {
        Err(ErrorKind::InvalidCardnum(cardnum.to_string()).into())
    }
}

pub fn validate_username(username: &str) -> Result<()> {
    lazy_static! {
        static ref USERNAME_REGEX: Regex = Regex::new("^[a-zA-Z0-9]+$").unwrap();
    }
    if USERNAME_REGEX.is_match(username) {
        Ok(())
    } else {
        Err(ErrorKind::InvalidUsername(username.to_string()).into())
    }
}
