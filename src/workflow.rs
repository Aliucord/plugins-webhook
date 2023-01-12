use std::env;

use lazy_static::lazy_static;
use reqwest::{Client, Request};
use serde_derive::Serialize;

use crate::Repository;

lazy_static! {
	static ref HTTP: Client = Client::new();
	static ref TOKEN: String = env::var("PLUGINS_REPO_TOKEN").expect("no PLUGINS_REPO_TOKEN env");
	static ref URL: String = format!(
		"https://api.github.com/repos/{}/actions/workflows/build-plugin.yml/dispatches",
		env::var("PLUGINS_REPO").expect("no PLUGINS_REPO env"),
	);
}

#[derive(Debug, Serialize)]
struct DispatchWorkflow {
	#[serde(rename = "ref")]
	ref_: String,
	inputs: WorkflowInputs,
}

#[derive(Debug, Serialize)]
struct WorkflowInputs {
	owner: String,
	repo: String,
}

pub async fn trigger_build(target_repo: Repository) -> Result<(), ()> {
	let req = HTTP.post(URL.to_string())
		.header("Authorization", format!("token {}", TOKEN.to_string()))
		.header("Content-Type", "application/json")
		.header("Accept", "application/vnd.github.v3+json")
		.json(&DispatchWorkflow {
			ref_: "main".to_string(),
			inputs: WorkflowInputs {
				owner: target_repo.owner.clone(),
				repo: target_repo.repo.clone(),
			},
		})
		.send().await;

	match req {
		Ok(_) => Ok(()),
		Err(e) => {
			log::error!("Failed to trigger build for {:?}: {:?}", target_repo, e);
			Err(())
		}
	}
}
