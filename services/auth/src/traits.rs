#[async_trait::async_trait]
pub trait ActiveRecord: Sized {
	type Pool;
	type Error;

	async fn save(&self, pool: &Self::Pool) -> Result<(), Self::Error>;
	async fn load(pool: &Self::Pool, id: &str) -> Result<Self, Self::Error>;
}
