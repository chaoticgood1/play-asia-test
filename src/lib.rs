use poem::{http::StatusCode, EndpointExt, Error, Response, Route};
use serde::{Serialize, Deserialize};

pub mod users;
pub mod items;

pub fn all_routes(data_path: String) -> Route {
  Route::new()
    .nest("/users", users::route())
    .nest("/", items::route().data(data_path.clone()))
}

fn response_json<T>(status_code: StatusCode, data: T) -> Response
where
  T: Serialize
{
  Response::builder()
    .status(status_code)
    .header("Content-Type", "application/json")
    .body(serde_json::to_string(&data)
    .unwrap())
}

fn error_response_json<T>(status_code: StatusCode, data: T) -> Error
where
  T: Serialize
{
  Error::from_response(
    Response::builder()
      .status(status_code)
      .header("Content-Type", "application/json")
        .body(serde_json::to_string(&data).unwrap()
      )
  )
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


