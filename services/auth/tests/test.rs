#[cfg(test)]
mod tests {
	use std::env;

	use auth_service::{db, models, traits::ActiveRecord};
	use sqlx::{migrate::MigrateDatabase, Pool, Sqlite};


	async fn setup_test_db() -> Result<Pool<Sqlite>, sqlx::Error> {
		// cleanup
		std::fs::remove_file("./tests/db/test.sqlite").unwrap_or_default();
		std::fs::remove_file("./tests/db/test.sqlite-shm").unwrap_or_default();
		std::fs::remove_file("./tests/db/test.sqlite-wal").unwrap_or_default();
		std::fs::File::create("./tests/db/test.sqlite").unwrap();
		// create test db
		// set env
		env::set_var("DATABASE_URL", "sqlite://./tests/db/test.sqlite");
		// create db
		sqlx::sqlite::Sqlite::create_database(&env::var("DATABASE_URL").unwrap());

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

	#[actix_rt::test]
	async fn test_user_model() {
		let usr = models::user::User {
			id:         "0".to_owned(),
			username:   "John".to_owned(),
			password:   "12345678".to_owned(),
			email:      "john@fmail.com".to_owned(),
			created_at: None,
		};

		assert_eq!(usr.id, "0".to_owned());
		assert_eq!(usr.username, "John".to_owned());
		assert_eq!(usr.password, "12345678".to_owned());
		assert_eq!(usr.email, "john@fmail.com".to_owned());

		let pool = setup_test_db()
			.await
			.unwrap();

		assert_eq!(
			usr.save(&pool)
				.await
				.unwrap(),
			()
		);

		let usr = models::user::User::load(&pool, "0")
			.await
			.unwrap();
		assert_eq!(usr.id, "0".to_owned());
		assert_eq!(usr.username, "John".to_owned());
		assert_eq!(usr.password, "12345678".to_owned());
		assert_eq!(usr.email, "john@fmail.com".to_owned());
		assert!(
			usr.created_at
				.is_some()
		);
		println!(
			"\n\n\ncreated_at: {}\n\n\n",
			usr.created_at
				.unwrap()
		);
	}
}
