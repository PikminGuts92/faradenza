#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

pub mod api;

pub use api::io::{get_files, filter_strings, read_from_csv};
pub use api::server::*;
pub use api::search::{Search};
