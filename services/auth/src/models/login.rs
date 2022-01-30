pub struct LoginData {
	pub email:    String,
	pub password: String,
}
impl LoginData {
	pub fn new(email: String, password: String) -> LoginData {
		LoginData { email, password }
	}
}
