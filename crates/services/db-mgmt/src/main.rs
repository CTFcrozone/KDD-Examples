// region:    --- Modules

mod config;
mod db;
mod error;

use db::init_dev_db;
pub use error::{Error, Result};
// use s3::S3ClientW;

use tracing_subscriber::EnvFilter;

// endregion: --- Modules

#[tokio::main]
async fn main() -> Result<()> {
	tracing_subscriber::fmt()
		.without_time()
		.with_target(false)
		.with_env_filter(EnvFilter::from_default_env())
		.init();

	// let cred = load_aws_cred_from_s3_config().await?;

	// println!("{:?}", s3_config());
	// println!("{:?}", cred);
	// let client = s3::s3_client::client_from_cred(cred)?;
	// let clientW = S3ClientW::from_s3_client(client);
	// clientW.create_bucket("dev-storage").await?;
	// clientW.create_bucket("prod-storage").await?;

	init_dev_db().await?;
	// println!("{:?}", &s3_config());

	// let buckets = clientW.list_buckets().await?;
	// println!("{:?}", buckets);

	// let files = clientW.list_keys("dev-storage").await?;
	// let f = clientW.list_keys_regex("dev-storage", r".*\.sql$").await?;

	// println!("{:?}", f);

	Ok(())
}
