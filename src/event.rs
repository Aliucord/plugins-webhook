use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};

const X_GITHUB_EVENT: &str = "X-GitHub-Event";

#[derive(Clone, Debug, PartialEq)]
pub enum GitHubEvent {
	Ping,
	Push,
	Create,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for GitHubEvent {
	type Error = ();

	async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
		let keys = request.headers().get(X_GITHUB_EVENT).collect::<Vec<_>>();
		if keys.len() != 1 {
			return Outcome::Failure((Status::BadRequest, ()));
		}

		let event = match keys[0] {
			"ping" => GitHubEvent::Ping,
			"push" => GitHubEvent::Push,
			"create" => GitHubEvent::Create,
			_ => { return Outcome::Failure((Status::Ok, ())); }
		};
		Outcome::Success(event)
	}
}
