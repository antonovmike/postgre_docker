#![allow(unused)]

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
	let pool = PgPoolOptions::new()
		.max_connections(5)
		.connect("postgres://postgres:welcome@localhost/postgres")
		.await?;

	Ok(())
}
