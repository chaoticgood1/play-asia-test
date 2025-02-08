use poem::{handler, http::StatusCode, post, web::Json, Error, Result, Route};
use serde::{Serialize, Deserialize};


pub fn route() -> Route {
  return Route::new()
    .at("/signup", post(sign_up))
}



#[handler]
async fn sign_up(user: Json<User>) -> Result<String> {
  println!("{:?}", user);

  return Result::Err(Error::from_string("In progress", StatusCode::BAD_REQUEST));
}


#[derive(Serialize, Deserialize, Debug)]
struct User {
  name: String,
  pass: String,
}