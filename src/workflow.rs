use std::env;

use lazy_static::lazy_static;
use log::error;
use reqwest::Client;
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
	owner_id: i32,
	repo_full_name: String,
}

pub async fn trigger_build(target_repo: Repository) -> Result<(), ()> {
	let req = HTTP.post(URL.to_string())
		.header("Authorization", format!("token {}", TOKEN.to_string()))
		.header("Content-Type", "application/json")
		.header("Accept", "application/vnd.github.v3+json")
		.header("User-Agent", "Aliucord/plugins-webhook")
		.json(&DispatchWorkflow {
			ref_: "main".to_string(),
			inputs: WorkflowInputs {
				repo_full_name: target_repo.full_name.clone(),
				owner_id: target_repo.owner.id,
			},
		})
		.send().await;

	match req {
		Ok(res) if res.status() != 200 => {
			error!(
				"Failed to trigger build on plugins repo for {}: {:?}",
				target_repo.full_name,
				res.text().await.unwrap_or("<failed to get body>".to_string())
			);
			Err(())
		},
		Ok(_) => Ok(()),
		Err(e) => {
			error!("Failed to trigger build on plugins repo for {}: {:?}", target_repo.full_name, e);
			Err(())
		}
	}
}