mod setup;


#[cfg(test)]
mod tests {
	use auth_service::{models, traits::ActiveRecord};
	use rs_uuid::uuid32;

	#[actix_rt::test]
	async fn test_user_model() {
		let uuid = uuid32();

		let usr = models::user::User {
			id:         None,
			uuid:       uuid.clone(),
			username:   "John".to_owned(),
			password:   "12345678".to_owned(),
			email:      "john@fmail.com".to_owned(),
			created_at: None,
		};

		assert_eq!(usr.id, None);
		assert_eq!(usr.uuid, uuid.clone());
		assert_eq!(usr.username, "John".to_owned());
		assert_eq!(usr.password, "12345678".to_owned());
		assert_eq!(usr.email, "john@fmail.com".to_owned());
		assert_eq!(usr.created_at, None);

		// save user
		let pool = super::setup::test_db()
			.await
			.unwrap();

		assert_eq!(
			usr.save(&pool)
				.await
				.unwrap(),
			()
		);

		// load user
		let usr = models::user::User::load(&pool, &uuid)
			.await
			.unwrap();
		assert_eq!(usr.id, Some(1_i64));
		assert_eq!(usr.uuid, uuid.clone());
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
