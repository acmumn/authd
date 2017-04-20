#![feature(custom_attribute)]
#![feature(custom_derive)]
#![feature(plugin)]
#![plugin(rocket_codegen)]
#![recursion_limit="128"]

extern crate crypto;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
#[macro_use]
extern crate error_chain;
extern crate jwt;
#[macro_use]
extern crate lazy_static;
extern crate publicsuffix;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate regex;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate time;
extern crate unicode_segmentation;

#[macro_use]
mod macros;

pub mod auth;
pub mod errors;
pub mod db;
pub mod models;
pub mod schema;
pub mod routes;
pub mod util;
