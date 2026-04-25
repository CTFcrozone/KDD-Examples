#![allow(unused)]
use crate::config::db_config;
use crate::{Error, Result};
use lazy_regex::{regex, Regex};
use lib_core::ctx::Ctx;
use lib_core::model::user::{User, UserBmc};
use lib_core::model::ModelManager;
use lib_utils::time::{format_time, now_utc};
use sqlx::postgres::{PgConnection, PgPoolOptions};
use sqlx::{Executor, Pool, Postgres};
use std::fs;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::Duration;
use tokio::process::Command;
use tracing::info;

type Db = Pool<Postgres>;

const SQL_RECREATE_DB_FILE_NAME: &str = "00-recreate-db.sql";
const SQL_DEV_SEED_FILE_NAME: &str = "02-dev-seed.sql";
const DEMO_PWD: &str = "e";

fn sql_base_dir() -> PathBuf {
	PathBuf::from(&db_config().SERVICE_SQL_FOLDER)
}

// needs cleanup, for now like this.
//
pub async fn init_dev_db() -> Result<()> {
	info!("{:<12} - init_dev_db()", "FOR-DEV-ONLY");

	let sql_root = sql_base_dir();
	let dev_initial_dir = sql_root.join("dev_initial");

	{
		let recreate_file = dev_initial_dir.join(SQL_RECREATE_DB_FILE_NAME);
		let root_db = new_db_pool(&db_config().SERVICE_PG_ROOT_URL).await?;

		pexec(&root_db, &recreate_file).await?;
	}

	let mut paths: Vec<PathBuf> = fs::read_dir(&dev_initial_dir)?
		.filter_map(|e| e.ok().map(|e| e.path()))
		.collect();
	paths.sort();

	let app_db = new_db_pool(&db_config().SERVICE_DB_URL).await?;
	for path in paths {
		let path_str = path.to_string_lossy();
		if path_str.ends_with(".sql")
			&& !path_str.ends_with(SQL_RECREATE_DB_FILE_NAME)
		{
			pexec_bulk(&app_db, &path).await?;
		}
	}

	// run_drop_upgrades().await?;

	let dev_seed_file = sql_root.join("dev_seed").join(SQL_DEV_SEED_FILE_NAME);
	let app_db = new_db_pool(&db_config().SERVICE_DB_URL).await?;
	pexec(&app_db, &dev_seed_file).await?;

	let mm = ModelManager::new().await.map_err(|_| Error::FailInit)?;
	let ctx = Ctx::root_ctx();

	let demo1_user: User = UserBmc::first_by_username(&ctx, &mm, "demo1")
		.await
		.map_err(|_| Error::FailInit)?
		.unwrap();
	UserBmc::update_pwd(&ctx, &mm, demo1_user.id, DEMO_PWD)
		.await
		.map_err(|_| Error::FailInit)?;

	Ok(())
}

pub async fn init_prod_db() -> Result<()> {
	// let files = s3_client.list_keys_regex("dev-storage", r".*\.sql$").await?;
	// let last_db_sql = files.last().ok_or_else(|| Error::NoSQLFiles)?;

	// let tmp_prod_sql_dir = Path::new(TMP_PROD_SQL_DIR);
	// let prod_file_name = Path::new(last_db_sql)
	// 	.file_name()
	// 	.ok_or_else(|| Error::FileNameIncorrectFormat)?;

	// if !tmp_prod_sql_dir.exists() {
	// 	create_dir_all(&tmp_prod_sql_dir);
	// }

	// s3_client.download_file("dev-storage", last_db_sql, tmp_prod_sql_dir).await?;

	run_drop_upgrades().await?;

	// pexec_bulk(&root_db, tmp_prod_sql_dir.join(prod_file_name).as_path()).await?;

	Ok(())
}

async fn run_drop_upgrades() -> Result<()> {
	let regex = Regex::new(r"drop-.*\.sql$")?;
	let upgrades_dir = sql_base_dir().join("upgrades");

	let mut paths: Vec<PathBuf> = fs::read_dir(upgrades_dir)?
		.filter_map(|e| e.ok().map(|e| e.path()))
		.collect();
	paths.sort();

	let app_db = new_db_pool(&db_config().SERVICE_DB_URL).await?;
	for path in paths {
		if regex.is_match(&path.to_string_lossy()) {
			pexec_bulk(&app_db, &path).await?;
		}
	}

	Ok(())
}

async fn pexec(db: &Db, file: &Path) -> Result<()> {
	info!("{:<12} - pexec: {file:?}", "FOR-DEV-ONLY");

	let content = fs::read_to_string(file)?;
	let sqls: Vec<&str> = content.split(';').collect();

	for sql in sqls {
		sqlx::query(sql).execute(db).await.map_err(|ex| {
			println!("pexec error while running:\n{sql}");
			println!("cause:\n{ex}");
			ex
		})?;
	}
	Ok(())
}

async fn pexec_bulk(db: &Db, file: &Path) -> Result<()> {
	info!("{:<12} - pexec: {file:?}", "FOR-DEV-ONLY");

	let content = fs::read_to_string(file)?;

	sqlx::raw_sql(&content).execute(db).await.map_err(|ex| {
		// println!("pexec error while running:\n{sql}");
		println!("cause:\n{ex}");
		ex
	})?;

	Ok(())
}

async fn new_db_pool(con_url: &str) -> Result<Db> {
	let db = PgPoolOptions::new()
		.max_connections(1)
		.acquire_timeout(Duration::from_millis(500))
		.connect(con_url)
		.await?;
	Ok(db)
}
