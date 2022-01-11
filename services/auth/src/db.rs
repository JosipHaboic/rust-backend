use sqlx::sqlite::SqlitePool;

use crate::{errors, user};


pub(crate) async fn initialize_database(pool: &SqlitePool) {
	// let mut connection = pool.acquire.await?;
	unimplemented!()
}

pub(crate) async fn add_user() -> Result<(), ()> {
	unimplemented!()
}
pub(crate) async fn find_user() -> Result<(), ()> {
	unimplemented!()
}
pub(crate) async fn remove_user() -> Result<(), ()> {
	unimplemented!()
}
