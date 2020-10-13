use serde::{Serialize, Deserialize};
use warp::{reject, Reply, Rejection};

use super::errors::ErrorVariant;

#[derive(Deserialize)]
pub struct UserParams {
  pub email: String,
  pub password: String
}

#[derive(Serialize)]
pub struct AuthInfo {
  pub token: String
}

#[derive(Serialize)]
pub struct ErrorResponse {
  pub message: String
}

// TODO: There is probably a way to refactor this to fully use warp rejections.
pub async fn auth_handler(
  db: crate::DbConnectionPool,
  data: UserParams
) -> std::result::Result<impl Reply, Rejection> {
  let client = super::database::get_db_conn(&db)
    .await
    .map_err(|e| reject::custom(e))?;

    super::auth::find_user(
      &client,
      &data.email
    )
    .await
    .map_err(|e| reject::custom(e))
    .map(|user|
      match super::auth::valid_password(
        &data.password,
        &user.encrypted_password
      ) {
        false => {
          Ok(
            warp::reply::with_status(
              warp::reply::json(&"{}"),
              warp::http::StatusCode::from_u16(403).unwrap()
            )
          )
        },
        true => {
          Ok(
            warp::reply::with_status(
              warp::reply::json(& AuthInfo { token: user.token }),
              warp::http::StatusCode::from_u16(200).unwrap()
            )
          )
        }
      }
    )
}

pub async fn rejection_handler(
  err: Rejection
) -> std::result::Result<impl Reply, std::convert::Infallible> {
  let code ;
  let message;

  if err.is_not_found() {
    code = warp::http::StatusCode::NOT_FOUND;
    message = "Not found";
  }  else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
    code = warp::http::StatusCode::BAD_REQUEST;
    message = "Invalid Body";
  } else if let Some(e) = err.find::<ErrorVariant>() {
    match e {
      ErrorVariant::DbPoolError(_) => {
        code = warp::http::StatusCode::SERVICE_UNAVAILABLE;
        message = "Database connection lost";
      }
      ErrorVariant::DbQueryError(_) => {
        code = warp::http::StatusCode::BAD_REQUEST;
        message = "Query error";
      }
    }
  } else {
    warn!("unhandled error: {:?}", err);
    code = warp::http::StatusCode::INTERNAL_SERVER_ERROR;
    message = "Internal Server Error";
  }

  let body = warp::reply::json(& ErrorResponse {
    message: message.into()
  });

  Ok(warp::reply::with_status(body, code))
}
