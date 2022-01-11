use sqlx::sqlite::SqlitePool;

use crate::{errors, user};


pub(crate) async fn initialize_database(pool: &SqlitePool) {
	// let mut connection = pool.acquire.await?;
	unimplemented!()
}

pub(crate) async fn add_user(user: user::User) -> Result<(), errors::AddUserError> {
	unimplemented!()
}
pub(crate) async fn remove_user(id: &str) -> Result<(), errors::RemoveUserError> {
	unimplemented!()
}
pub(crate) async fn find_user(id: &str) -> Result<User, errors::FindUserError> {
	unimplemented!()
}
