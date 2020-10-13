use mobc_postgres::{tokio_postgres, PgConnectionManager};
use std::time::Duration;
use std::env;

use crate::{DbConnectionPool, DbConnection};
use super::errors::ErrorVariant;

const DB_POOL_MAX_OPEN: u64 = 32;
const DB_POOL_MAX_IDLE: u64 = 8;
const DB_POOL_TIMEOUT_SECONDS: u64 = 15;

pub fn create_pool()
-> std::result::Result<DbConnectionPool, mobc::Error<tokio_postgres::Error>> {
  let db_host = match env::var("DB_HOST") {
    Ok(val) => val,
    Err(_) => panic!("Missing DB_HOST variable")
  };

  let db_port: u16 = match env::var("DB_PORT") {
    Ok(val) => val.parse().unwrap(),
    Err(_) => panic!("Missing DB_PORT variable")
  };

  let db_user = match env::var("DB_USER") {
    Ok(val) => val,
    Err(_) => panic!("Missing DB_USER variable")
  };

  let db_password = match env::var("DB_PASSWORD") {
    Ok(val) => val,
    Err(_) => panic!("Missing DB_PASSWORD variable")
  };

  let db_name = match env::var("DB_NAME") {
    Ok(val) => val,
    Err(_) => panic!("Missing DB_NAME variable")
  };

  let mut config = tokio_postgres::Config::new();
  config.user(&db_user[..]);
  config.password(&db_password[..]);
  config.host(&db_host[..]);
  config.port(db_port);
  config.dbname(&db_name[..]);

  let manager = PgConnectionManager::new(
    config, tokio_postgres::NoTls
  );

  Ok(
    mobc::Pool::builder()
      .max_open(DB_POOL_MAX_OPEN)
      .max_idle(DB_POOL_MAX_IDLE)
      .get_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)))
      .build(manager)
  )
}

pub async fn get_db_conn(db_pool: &DbConnectionPool) -> Result<DbConnection, super::errors::ErrorVariant> {
  db_pool
    .get()
    .await
    .map_err(ErrorVariant::DbPoolError)
}
