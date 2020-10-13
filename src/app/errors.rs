use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorVariant {
  #[error("Database pool error")]
  DbPoolError(mobc::Error<tokio_postgres::Error>),
  #[error("Database query error")]
  DbQueryError(#[from] tokio_postgres::Error)
}

impl warp::reject::Reject for ErrorVariant {}
