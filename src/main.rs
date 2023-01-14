#[macro_use]
extern crate rocket;


use log::info;
use rocket::http::Status;
use rocket::Request;
use serde_derive::Deserialize;

use crate::event::GitHubEvent;
use crate::verification::SignedPayload;

mod event;
mod verification;
mod workflow;

#[launch]
fn rocket() -> _ {
	rocket::build()
		.mount("/", routes![
			root,
			webhook,
		])
		.register("/", catchers![
			catcher_default,
		])
}

#[catch(default)]
fn catcher_default(_: Status, _: &Request<'_>) -> &'static str {
	"Unknown Error"
}

#[get("/")]
fn root() -> &'static str {
	"read if cute"
}

#[post("/github", data = "<payload>")]
async fn webhook(event: GitHubEvent, payload: SignedPayload) -> Status {
	let handler = match event {
		GitHubEvent::Push => handle_push(payload.0).await,
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
pub struct User {
	login: String,
}

#[derive(Debug, Deserialize)]
pub struct Repository {
	id: i32,
	name: String,
	owner: User,
	private: bool,
	disabled: bool,
	archived: bool,
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

	// Check for valid release branch
	if data.ref_ != "refs/heads/release"
		|| data.deleted
		|| data.repository.archived
		|| data.repository.private
		|| data.repository.disabled
	{
		return Ok(());
	}

	info!("Received release branch push on {}/{}", &data.repository.owner.login, &data.repository.name);

	match workflow::trigger_build(data.repository).await {
		Ok(_) => Ok(()),
		Err(_) => Err(()),
	}
}
