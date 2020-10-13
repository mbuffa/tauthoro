extern crate pretty_env_logger;
#[macro_use] extern crate log;

use warp::Filter;
use mobc_postgres::{tokio_postgres};
use tokio_postgres::NoTls;

type DbConnectionManager = mobc_postgres::PgConnectionManager<NoTls>;
type DbConnection = mobc::Connection<DbConnectionManager>;
type DbConnectionPool = mobc::Pool<DbConnectionManager>;

mod app;

#[tokio::main]
async fn main() {
  pretty_env_logger::init();

  let db_pool = app::database::create_pool().expect("database pool can't be created");

  let heartbeat = warp::path!("monitoring" / "heartbeat")
    .and(warp::get())
    .map(warp::reply)
    .map(|reply| {
      info!("GET /monitoring/heartbeat 204");
      warp::reply::with_status(
        reply,
        warp::http::StatusCode::from_u16(204).unwrap()
      )
    });

  let auth = warp::path("auth")
    .and(warp::post())
    .and(with_db(db_pool.clone()))
    .and(warp::body::json())
    .and_then(app::handlers::auth_handler);

  let routes = heartbeat
    .or(auth)
    .recover(app::handlers::rejection_handler);

  debug!("Serving on 0.0.0.0:8080");

  warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}

fn with_db(
  db_pool: DbConnectionPool
) -> impl warp::Filter<Extract = (DbConnectionPool,), Error = std::convert::Infallible> + Clone {
  warp::any().map(move || db_pool.clone())
}
