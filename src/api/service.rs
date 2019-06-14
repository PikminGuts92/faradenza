extern crate hyper;

use hyper::*;
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

pub struct AddressService {

}

impl AddressService {
	pub fn new() -> AddressService {
		AddressService {

		}
	}

	pub fn run(&self) -> Result<()> {
		let addr = ([127, 0, 0, 1], 3000).into();
		let test = Arc::new(vec![2,3]);

		let router = move || {
			let t = test.clone();
			

			service_fn_ok(move |req| {
				route_request(req)
			})
		};

		let server = Server::bind(&addr)
			.serve(router)
			.map_err(|e| eprintln!("Server error: {}", e));

		// Run server
		hyper::rt::run(server);
		Ok(())
	}
}

fn route_request(req: Request<Body>) -> Response<Body> {
	let path_string: String = req.uri().path().to_owned();
	let mut paths: Vec<&str> = path_string.split("/").collect();
	paths.retain(|&p| p != ""); // Removes empty results (issue #33882)
	let first_path: &str = match &paths.get(0) {
		Some(value) => value,
		None => ""
	};
	let second_path: &str = match &paths.get(1) {
		Some(value) => value,
		None => ""
	};
	let res = match (req.method(), first_path) {
		(&Method::GET, "stats") => {
			// TODO: Get stats from service
			Response::builder()
				.header(header::CONTENT_TYPE, "application/json")
				.body(Body::from("statistics"))
				.unwrap()
		},
		(&Method::GET, "frequency") => {
			// TODO: Get stats from service
			Response::builder()
				.header(header::CONTENT_TYPE, "application/json")
				.body(Body::from(second_path.to_owned()))
				.unwrap()
		},
		_ => {
			Response::builder()
				.status(StatusCode::NOT_FOUND)
				.body(Body::empty())
				.unwrap()
		},
	};
	res
}