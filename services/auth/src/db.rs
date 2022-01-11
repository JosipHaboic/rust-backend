use sqlx::sqlite::SqlitePool;
use crate::user;
use crate::errors;



pub (crate) async fn initialize_database(pool: &SqlitePool) {
	let mut connection = pool.acquire.await?;

	sqlx::query!("CREATE TABLE IF NOT EXISTS users\n\
	id TEXT PRIMARY KEY NOT NULL,\n\
	username TEXT NOT NULL,\n\
	password TEXT NOT NULL");
}

pub (crate) add_user(user: user::User) -> Result<(), errors::AddUserError> { unimplemented!() }
pub (crate) remove_user(id: &str) -> Result<(), errors::RemoveUserError> { unimplemented!() }
pub (crate) find_user(id: &str) -> Result<User, errors::FindUserError> { unimplemented!() }