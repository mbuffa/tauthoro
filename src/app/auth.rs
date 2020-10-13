extern crate bcrypt;

use crate::app::errors::ErrorVariant;
use crate::app::records::User;
use crate::DbConnection;

pub async fn find_user(
  client: &DbConnection,
  email: &String
) -> Result<User, ErrorVariant> {
  client.query_one(
    "SELECT * FROM find_user_and_token($1);", &[email]
  )
  .await
  .map_err(ErrorVariant::DbQueryError)
  .map(|row| User {
    id: row.get(0),
    encrypted_password: row.get(1),
    token: row.get(2)
  })
}

pub fn valid_password(password: &String, encrypted_password: &String) -> bool {
  match bcrypt::verify(password, encrypted_password) {
    Ok(bool) => bool,
    Err(_) => false
  }
}
