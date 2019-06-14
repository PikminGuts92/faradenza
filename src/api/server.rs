extern crate rocket;

use crate::api::search::Search;
use rocket::State;
use rocket::response::content;
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}, Mutex};

/*
impl AddressService {
	/*
	pub fn new(search: &Search) -> AddressService {
		AddressService {
			search: search
		}
	}*/

	pub fn run(&self, search: &Search) {
		let rocket = rocket::ignite()
			.mount("", routes![get_stats, get_frequency)
			.manage(&self);

		rocket.launch();

		/*
		let addr = ([127, 0, 0, 1], 3000).into();

		//let search2: &'a Search = &self.search;
		
		//let async_search = Arc::new(Mutex::new(search));
		//let async_search = Arc::new(Mutex::new(&self.search));
		
		let router = move || {
			//let clone_search = async_search.clone();
			
			service_fn_ok(move |req| {
				// route_request(&self.search, req)
				Response::builder()
					.status(StatusCode::NOT_FOUND)
					.body(Body::empty())
					.unwrap()
			})
		};

		let server = Server::bind(&addr)
			.serve(router)
			.map_err(|e| eprintln!("Server error: {}", e));

		// Run server
		hyper::rt::run(server);*/
	}
}
*/
/*
fn route_request<'a>(search: Arc<Mutex<&'a Search>>, req: Request<Body>) -> Response<Body> {
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
*/

pub fn run_server(search: Search) {
	let rocket = rocket::ignite()
			.mount("/", routes![
				get_index,
				get_stats,
				get_frequency])
			.manage(search);

		rocket.launch();
}

#[get("/")]
fn get_index() -> String {
	"This is the root!".to_owned()
}

#[get("/stats")]
fn get_stats(search: State<Search>) -> String {
	"9001".to_owned()
}

#[get("/frequency/<term>")]
fn get_frequency(term: String, search: State<Search>) -> String {
	//"1234".to_owned()
	search.get_data()
}