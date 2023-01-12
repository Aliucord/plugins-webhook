use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha1::Sha1;
use data::Outcome;
use rocket::{Data, data, Request};
use rocket::data::{FromData, ToByteUnit};
use rocket::http::Status;
use rocket::tokio::io::AsyncReadExt;

// Adapted from https://github.com/aergonaut/railgun/blob/213c546da9b79786d38f18ee67bdd2ab73034232/src/railgun/request.rs

#[derive(Debug, PartialEq)]
pub struct SignedPayload(pub String);

const X_HUB_SIGNATURE: &str = "X-Hub-Signature";

#[rocket::async_trait]
impl<'r> FromData<'r> for SignedPayload {
	type Error = ();

	async fn from_data(request: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self, Self::Error> {
		if !request.headers().get("Content-Type").collect::<Vec<_>>().contains(&"application/json") {
			return Outcome::Failure((Status::BadRequest, ()))
		}

		let keys = request.headers().get(X_HUB_SIGNATURE).collect::<Vec<_>>();
		if keys.len() != 1 {
			return Outcome::Failure((Status::BadRequest, ()));
		}

		let signature = keys[0];

		let mut body = Vec::new();
		if let Err(_) = data.open(1.mebibytes()).read(&mut body).await {
			return Outcome::Failure((Status::InternalServerError, ()));
		}

		let secret = match std::env::var("WEBHOOK_SECRET") {
			Ok(s) => s,
			Err(_) => { return Outcome::Failure((Status::InternalServerError, ())); }
		};

		if !is_valid_signature(&signature, &body, &secret) {
			return Outcome::Failure((Status::BadRequest, ()));
		}

		return match String::from_utf8(body) {
			Ok(str) => Outcome::Success(SignedPayload(str)),
			Err(_) => Outcome::Failure((Status::BadRequest, ()))
		}
	}
}

fn is_valid_signature(signature: &str, body: &[u8], secret: &str) -> bool {
	let digest = Sha1::new();
	let mut hmac = Hmac::new(digest, secret.as_bytes());
	hmac.input(body);
	let expected_signature = hmac.result();

	let parts = signature.splitn(2, '=').collect::<Vec<_>>();
	let code = parts[1];

	crypto::util::fixed_time_eq(bytes_to_hex(expected_signature.code()).as_bytes(), code.as_bytes())
}

const CHARS: &'static [u8] = b"0123456789abcdef";

fn bytes_to_hex(bytes: &[u8]) -> String {
	let mut v = Vec::with_capacity(bytes.len() * 2);
	for &byte in bytes {
		v.push(CHARS[(byte >> 4) as usize]);
		v.push(CHARS[(byte & 0xf) as usize]);
	}

	unsafe {
		String::from_utf8_unchecked(v)
	}
}
