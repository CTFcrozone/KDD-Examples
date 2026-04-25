use lib_utils::envs::get_env;
use std::sync::OnceLock;

pub fn db_config() -> &'static DbConfig {
	static INSTANCE: OnceLock<DbConfig> = OnceLock::new();

	INSTANCE.get_or_init(|| {
		DbConfig::load_from_env().unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}"))
	})
}

#[allow(non_snake_case)]
pub struct DbConfig {
	pub SERVICE_DB_URL: String,
	pub SERVICE_PG_ROOT_URL: String,
	pub SERVICE_SQL_FOLDER: String,
}

impl DbConfig {
	fn load_from_env() -> lib_utils::envs::Result<DbConfig> {
		Ok(DbConfig {
			SERVICE_DB_URL: get_env("SERVICE_DB_URL")?,
			SERVICE_PG_ROOT_URL: get_env("SERVICE_PG_ROOT_URL")?,
			SERVICE_SQL_FOLDER: get_env("SERVICE_SQL_FOLDER")?,
		})
	}
}
