use rs_uuid::uuid32;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, Row};

use crate::traits::ActiveRecord;


#[derive(Debug, Deserialize, Serialize)]
pub struct User {
	pub id:         Option<i64>,
	pub uuid:       String,
	pub username:   String,
	pub password:   String,
	pub email:      String,
	pub created_at: Option<String>,
}
impl User {
	pub fn new(username: &str, password: &str, email: &str) -> User {
		User {
			id:         None,
			uuid:       uuid32(),
			username:   username.to_owned(),
			password:   password.to_owned(),
			email:      email.to_owned(),
			created_at: None,
		}
	}
}

#[async_trait::async_trait]
impl ActiveRecord for User {
	type Error = sqlx::Error;
	type Pool = SqlitePool;

	async fn save(&self, pool: &SqlitePool) -> Result<(), sqlx::Error> {
		match sqlx::query(
			"INSERT INTO user (uuid, username, password, email) VALUES (?1, ?2, ?3, ?4)",
		)
		.bind(&self.uuid)
		.bind(&self.username)
		.bind(&self.password)
		.bind(&self.email)
		.execute(pool)
		.await
		{
			Ok(_) => Ok(()),
			Err(error) => Err(error),
		}
	}

	async fn load(pool: &SqlitePool, id: &str) -> Result<User, sqlx::Error> {
		match sqlx::query("SELECT * FROM user WHERE uuid = ?1")
			.bind(id)
			.fetch_one(pool)
			.await
		{
			Ok(row) => Ok(User {
				id:         row.get("id"),
				uuid:       row.get("uuid"),
				username:   row.get("username"),
				password:   row.get("password"),
				email:      row.get("email"),
				created_at: row.get("created_at"),
			}),
			Err(error) => Err(error),
		}
	}
}
