use std::env;

use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha256;
use data::Outcome;
use lazy_static::lazy_static;
use rocket::{Data, data, Request};
use rocket::data::{FromData, ToByteUnit};
use rocket::http::Status;
use rocket::tokio::io::AsyncReadExt;

lazy_static! {
	static ref SECRET: String = env::var("WEBHOOK_SECRET").expect("no WEBHOOK_SECRET env");
}

#[derive(Debug, PartialEq)]
pub struct SignedPayload(pub String);

const X_HUB_SIGNATURE_256: &str = "X-Hub-Signature-256";

#[rocket::async_trait]
impl<'r> FromData<'r> for SignedPayload {
	type Error = &'r str;

	async fn from_data(request: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self, Self::Error> {
		if !request.headers().get("Content-Type").all(|h| h == "application/json") {
			return Outcome::Failure((Status::BadRequest, "Invalid content type"))
		}

		let signature = match request.headers().get(X_HUB_SIGNATURE_256).next() {
			Some(s) => s,
			None => {
				return Outcome::Failure((Status::BadRequest, "Missing signature header"));
			}
		};

		let mut body = String::new();
		if let Err(_) = data.open(1.mebibytes()).read_to_string(&mut body).await {
			return Outcome::Failure((Status::InternalServerError, "Failed to read body"));
		}

		if !is_valid_signature_256(&signature, &body, &SECRET) {
			return Outcome::Failure((Status::BadRequest, "Invalid signature"));
		}

		return Outcome::Success(SignedPayload(body));
	}
}

fn is_valid_signature_256(signature: &str, body: &str, secret: &str) -> bool {
	let mut hmac = Hmac::new(Sha256::new(), secret.as_bytes());
	hmac.input(body.as_bytes());

	let calculated_sig = format!("sha256={}", bytes_to_hex(hmac.result().code()));

	crypto::util::fixed_time_eq(calculated_sig.as_bytes(), signature.as_bytes())
}

fn bytes_to_hex(bytes: &[u8]) -> String {
	const CHARS: &'static [u8] = b"0123456789abcdef";

	let mut v = Vec::with_capacity(bytes.len() * 2);
	for &byte in bytes {
		v.push(CHARS[(byte >> 4) as usize]);
		v.push(CHARS[(byte & 0xf) as usize]);
	}

	unsafe {
		String::from_utf8_unchecked(v)
	}
}
