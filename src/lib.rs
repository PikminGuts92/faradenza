#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

pub mod api;

pub use api::io::*;
pub use api::server::*;
pub use api::search::{Search};
