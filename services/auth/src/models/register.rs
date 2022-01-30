pub struct RegistrationData {
	pub username: String,
	pub password: String,
	pub email:    String,
}
impl RegistrationData {
	pub fn new(username: String, password: String, email: String) -> RegistrationData {
		RegistrationData {
			username,
			password,
			email,
		}
	}
}
