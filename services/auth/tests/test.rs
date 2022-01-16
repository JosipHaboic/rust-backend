#[cfg(test)]
mod tests {
	use std::{env, process::Command};

	use auth_service::{db, models, traits::ActiveRecord};
	use sqlx::{Pool, Sqlite};


	fn create_test_user() -> models::user::User {
		models::user::User {
			id:         "0".to_owned(),
			username:   "John".to_owned(),
			password:   "12345678".to_owned(),
			email:      "john@fmail.com".to_owned(),
			created_at: None,
		}
	}

	fn setup_test_db() {
		// env::set_var("DATABASE_URL", "sqlite://./tests/db/test.sqlite");

		let _output = if cfg!(target_os = "windows") {
			Command::new("cmd")
				.args([
					"SET DATABASE_URL=sqlite://./tests/db/test.sqlite",
					"sqlx db create",
					"sqlx migrate run",
				])
				.output()
				.expect("failed to execute process\n\n\n")
		}
		else {
			Command::new("sh")
				.arg("EXPORT DATABASE_URL=sqlite://./tests/db/test.sqlite")
				.arg("sqlx db create")
				.arg("sqlx migrate run")
				.output()
				.expect("failed to execute process\n\n\n")
		};
	}

	async fn create_test_pool() -> Result<Pool<Sqlite>, sqlx::Error> {
		setup_test_db();

		match db::create_pool().await {
			Ok(pool) => {
				run_test_migrations(&pool)
					.await
					.unwrap();

				Ok(pool)
			},
			Err(error) => Err(error),
		}
	}

	async fn run_test_migrations(pool: &Pool<Sqlite>) -> Result<(), sqlx::migrate::MigrateError> {
		sqlx::migrate!("./migrations")
			.run(pool)
			.await
	}

	#[actix_rt::test]
	async fn test_create_pool() {
		setup_test_db();
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
		setup_test_db();
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
	}

	#[actix_rt::test]
	async fn test_user_model_load() {
		setup_test_db();
		let pool = create_test_pool()
			.await
			.unwrap();
		let usr = models::user::User::load(&pool, "0")
			.await
			.unwrap();

		assert_eq!(usr.id, "0".to_owned());
		assert_eq!(usr.username, "John".to_owned());
		assert_eq!(usr.password, "12345678".to_owned());
		assert_eq!(usr.email, "john@fmial.com".to_owned());
	}
}
