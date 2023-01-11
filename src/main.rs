#[macro_use]
extern crate rocket;

use crate::event::GitHubEvent;

mod event;
mod verification;

#[launch]
fn rocket() -> _ {
	rocket::build().mount("/", routes![
		root,
		webhook,
	])
}

#[get("/")]
fn root() -> &'static str {
	"read if cute"
}

#[post("/github", data = "<payload>")]
fn webhook(event: GitHubEvent, payload: i32) -> &'static str {
	"Hello, world!"
}
