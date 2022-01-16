#[cfg(test)]
mod tests {
	use std::env;

	use auth_service::{db, models, traits::ActiveRecord};
	use sqlx::{migrate::MigrateDatabase, Error, Pool, Sqlite};


	// helper fn
	fn create_test_user() -> models::User {
		models::User {
			id:         "0".to_owned(),
			username:   "John".to_owned(),
			password:   "12345678".to_owned(),
			email:      "john@fmail.com".to_owned(),
			created_at: None,
		}
	}
	async fn create_test_pool() -> Result<Pool<Sqlite>, Error> {
		// SET DATABASE_URL=sqlite://./sqlite/db/main.db
		// sqlx db create
		// sqlx migrate run
		db::create_pool().await
	}
	async fn run_test_migrations(pool: &Pool<Sqlite>) -> Result<(), sqlx::migrate::MigrateError> {
		sqlx::migrate!("./migrations")
			.run(pool)
			.await
	}

	#[actix_rt::test]
	pub async fn a_initialize() {
		env::set_var("DATABASE_URI", "sqlite://./tests/db/test.db");
		let pool = create_test_pool()
			.await
			.unwrap();
		sqlx::sqlite::Sqlite::create_database(&env::var("DATABASE_URI").unwrap())
			.await
			.unwrap();
		run_test_migrations(&pool)
			.await
			.unwrap();

		println!("Done setup for tests.");
		assert!(true);
	}

	#[actix_rt::test]
	async fn test_create_pool() {
		let pool = create_test_pool().await;
		assert!(pool.is_ok());
	}

	#[actix_rt::test]
	async fn test_user_model_new() {
		let usr = create_test_user();
		assert_eq!(usr.id, "0".to_owned());
		assert_eq!(usr.username, "John".to_owned());
		assert_eq!(usr.password, "12345678".to_owned());
		assert_eq!(usr.email, "john@fmail.com".to_owned());
	}

	#[actix_rt::test]
	async fn test_user_model_save() {
		let usr = create_test_user();
		let pool = create_test_pool()
			.await
			.unwrap();
		assert_eq!(
			usr.save(&pool)
				.await
				.unwrap(),
			()
		);
		env::remove_var("DATABASE_URI");
	}

	#[actix_rt::test]
	async fn test_user_model_load() {
		let pool = create_test_pool()
			.await
			.unwrap();
		let usr = models::User::load(&pool, "0")
			.await
			.unwrap();

		assert_eq!(usr.id, "0".to_owned());
		assert_eq!(usr.username, "John".to_owned());
		assert_eq!(usr.password, "12345678".to_owned());
		assert_eq!(usr.email, "john@fmial.com".to_owned());
	}
}
