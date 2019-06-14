extern crate rocket;
extern crate serde_derive;

use crate::api::search::Search;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

#[derive(Serialize, Deserialize)]
struct Stats {
	terms: Vec<String>
}

pub fn run_server(search: Search) {
	let rocket = rocket::ignite()
			.mount("/", routes![
				get_index,
				get_stats,
				get_frequency])
			.register(catchers![not_found])
			.manage(search);

		rocket.launch();
}

#[get("/")]
fn get_index() -> String {
	"This is the root!".to_owned()
}

#[get("/stats", format = "json")]
fn get_stats(search: State<Search>) -> Option<Json<Stats>> {
	Option::Some(Json(Stats {
		terms: search.get_terms_copied(),
	}))
}

#[get("/frequency/<term>")]
fn get_frequency(term: String, search: State<Search>) -> String {
	format!["{}", search.get_frequency(&term)]
}

#[catch(404)]
fn not_found() -> JsonValue {
  json!({
      "status": "error",
      "reason": "Resource was not found."
  })
}