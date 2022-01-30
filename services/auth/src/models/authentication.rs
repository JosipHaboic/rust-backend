use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;

use crate::traits::{ActiveRecord, IdentityField};



#[derive(Debug, Deserialize, Serialize)]
pub struct Authentication {
	id:       Option<i64>,
	ref_uuid: String,
	token:    Option<String>,
}

impl Authentication {
	pub fn new(ref_uuid: &str) -> Authentication {
		Authentication {
			id:       None,
			ref_uuid: ref_uuid.to_owned(),
			token:    None,
		}
	}
}

impl IdentityField for Authentication {
	type IdType = String;

	fn identity(self: Self) -> Self::IdType {
		self.ref_uuid
	}
}

#[async_trait::async_trait]
impl ActiveRecord for Authentication {
	type Error = sqlx::Error;
	type Pool = SqlitePool;

	async fn save(&self, _pool: &Self::Pool) -> Result<(), Self::Error> {
		unimplemented!()
	}

	async fn load(_pool: &Self::Pool, _id: &str) -> Result<Self, Self::Error> {
		unimplemented!()
	}
}
