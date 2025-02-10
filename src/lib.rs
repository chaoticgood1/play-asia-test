use poem::{EndpointExt, Route};
use serde::{Serialize, Deserialize};

pub mod users;
pub mod items;

pub fn all_routes(data_path: String) -> Route {
  Route::new()
    .nest("/users", users::route())
    .nest("/", items::route().data(data_path.clone()))
}

static SECRET_KEY: &str = "SuperSecretKey";

#[derive(Serialize, Deserialize, Debug)]
struct ErrorResponse {
  error: String,
  msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
  sub: String,
  exp: usize
}


