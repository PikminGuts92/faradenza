extern crate hyper;

use hyper::*;
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use std::env;

fn hello_world(req: Request<Body>) -> Response<Body> {
	//println!("Path: {}", req.uri().path());
	//println!("Query {}", req.uri().query().unwrap());

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

    //Response::new(Body::from("HELLO WORLD"))
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

fn main() {
	let arguments: Vec<String> = env::args().collect();

	if arguments.len() <= 1 {
		println!("Usage: faradenza.exe D:\\openaddr-collected-us_northeast");
		return;
	}

	faradenza::open_dir(&arguments[1]);

	// stats
	// frequency/%term

	//faradenza::open_dir("D:\\openaddr-collected-us_northeast");
	let addr = ([127, 0, 0, 1], 3000).into();
	let server = Server::bind(&addr)
		.serve(|| service_fn_ok(hello_world))
		.map_err(|e| eprintln!("Server error: {}", e));
	
	// Run server
	hyper::rt::run(server);

	println!("Server started!");
}