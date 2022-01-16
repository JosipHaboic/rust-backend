pub mod time {

	pub fn timestamp() -> String {
		chrono::Local::now()
			.format("%Y-%m-%d--%A %H:%M:%S")
			.to_string()
	}
}

pub mod secret {

	pub fn hash(pwd: String) -> Result<String, bcrypt::BcryptError> {
		bcrypt::hash(pwd, bcrypt::DEFAULT_COST)
	}

	pub fn validate(pwd: &str, hashed: &str) -> Result<bool, bcrypt::BcryptError> {
		bcrypt::verify(pwd, hashed)
	}
}
