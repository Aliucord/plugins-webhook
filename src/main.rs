#[macro_use]
extern crate rocket;

use std::future::Future;

use log::info;
use rocket::http::Status;
use serde_derive::Deserialize;

use crate::event::GitHubEvent;
use crate::verification::SignedPayload;

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
async fn webhook(event: GitHubEvent, payload: SignedPayload) -> Status {
	let handler = match event {
		GitHubEvent::Push => handle_push(payload.0).await,
		GitHubEvent::Create => handle_create(payload.0).await,
		GitHubEvent::Ping => handle_ping().await,
	};

	match handler {
		Ok(_) => Status::Ok,
		Err(_) => Status::InternalServerError,
	}
}

async fn handle_ping() -> Result<(), ()> {
	info!("Received ping!");
	return Ok(());
}

#[derive(Debug, Deserialize)]
struct Repository {
	owner: String,
	repo: String,
}

#[derive(Debug, Deserialize)]
struct PushEventData {
	deleted: bool,
	repository: Repository,
	#[serde(rename = "ref")]
	ref_: String,
}

async fn handle_push(body: String) -> Result<(), ()> {
	let data = match serde_json::from_str::<PushEventData>(body.as_str()) {
		Ok(d) => d,
		Err(_) => { return Err(()); }
	};

	return Ok(());
}

// #[derive(Debug, Deserialize)]
// struct CreateEventData {
// 	#[serde(rename = "ref")]
// 	ref_: String,
// }
//
// async fn handle_create(body: String) -> Result<(), ()> {
// 	let data = match serde_json::from_str::<CreateEventData>(body.as_str()) {
// 		Ok(d) => d,
// 		Err(_) => { return Err(()); }
// 	};
//
// 	return Ok(());
// }
