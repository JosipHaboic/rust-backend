use jsonwebtoken::{encode, EncodingKey};
use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
	login_session: String,
	scope:         Option<String>,
	user:          String,
	uuid:          String,
	// -
	exp:           String,
	iat:           String,
	iss:           String,
	nbf:           String,
	sub:           String,
}

pub fn token() -> jsonwebtoken::errors::Result<String> {
	let mut header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS512);

	header.typ = Some("JWT".to_owned());
	header.cty = Some("JSON".to_owned());
	header.kid = Some(rs_uuid::uuid32());

	println!("token header: {:?}", &header);

	let claims = Claims {
		login_session: rs_uuid::uuid32(),
		scope:         None,
		user:          "John Wayne".to_owned(),
		uuid:          rs_uuid::uuid32(),
		// -
		exp:           crate::util::time::expire_in(60 * 12),
		iat:           crate::util::time::timestamp(),
		iss:           "Andromeda Systems API".to_owned(),
		nbf:           crate::util::time::timestamp(),
		sub:           "You".to_owned(),
	};

	encode(
		&header,
		&claims,
		&EncodingKey::from_secret("this will be in .env file".as_ref()),
	)
}
