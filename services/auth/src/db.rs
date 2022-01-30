#![allow(dead_code)]

use std::env;

use sqlx::sqlite::SqlitePool;



pub async fn create_pool() -> Result<SqlitePool, sqlx::Error> {
	log::info!(target: "auth-service", "DB: Created sqlite pool");

	SqlitePool::connect(&env::var("DATABASE_URL").unwrap()).await
}
