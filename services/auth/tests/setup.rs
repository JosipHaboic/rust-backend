use std::env;

use auth_service::db;
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite};


pub async fn test_db() -> Result<Pool<Sqlite>, sqlx::Error> {
	// cleanup
	std::fs::remove_file("./tests/db/test.sqlite").unwrap_or_default();
	std::fs::remove_file("./tests/db/test.sqlite-shm").unwrap_or_default();
	std::fs::remove_file("./tests/db/test.sqlite-wal").unwrap_or_default();
	std::fs::File::create("./tests/db/test.sqlite").unwrap();
	// set env
	env::set_var("DATABASE_URL", "sqlite://./tests/db/test.sqlite");
	// create db
	sqlx::sqlite::Sqlite::create_database(&env::var("DATABASE_URL").unwrap());
	// run migrations
	match db::create_pool().await {
		Ok(pool) => {
			sqlx::migrate!("./migrations")
				.run(&pool)
				.await
				.unwrap();

			Ok(pool)
		},
		Err(error) => Err(error),
	}
}
