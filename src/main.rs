extern crate hyper;

//use faradenza::*;
use hyper::*;
use hyper::rt::Future;
use hyper::service::service_fn_ok;

fn hello_world(req: Request<Body>) -> Response<Body> {
	//println!("Path: {}", req.uri().path());
	//println!("Query {}", req.uri().query().unwrap());

	let pathString = req.uri().path().to_owned();
	let mut paths: Vec<&str> = pathString.split("/").collect();
	paths.retain(|&p| p != ""); // Removes empty results (issue #33882)

	let firstPath = match &paths.get(0) {
		Some(value) => value,
		None => ""
	};

	let secondPath = match &paths.get(1) {
		Some(value) => value,
		None => ""
	};
	
    //Response::new(Body::from("HELLO WORLD"))
	let res = match (req.method(), firstPath) {
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
				.body(Body::from(secondPath))
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