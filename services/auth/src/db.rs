use sqlx::{sqlite::Sqlite, Pool};


pub(crate) fn create_pool(
	uri: &'static str,
) -> impl std::future::Future<Output = Result<Pool<Sqlite>, sqlx::Error>> {
	Pool::<Sqlite>::connect(uri)
}
