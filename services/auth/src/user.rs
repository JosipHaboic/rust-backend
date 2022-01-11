use crate::role;


#[derive(Clone)]
pub(crate) struct User {
	pub uuid:     String,
	pub email:    String,
	pub username: String,
	pub pw:       String,
	pub role:     role::Role,
}
