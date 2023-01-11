#[macro_use]
extern crate rocket;

use crate::event::GitHubEvent;

mod event;
mod verification;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        root,
    ])
}

#[get("/")]
fn root() -> &'static str {
    "read if cute"
}
