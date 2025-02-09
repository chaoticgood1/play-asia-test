use poem::{listener::TcpListener, Route, Server};
use serde::{Serialize, Deserialize};


mod users;
mod items;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
  let app = Route::new()
    .nest("/users", users::route())
    .nest("/", items::route());

  Server::new(TcpListener::bind("0.0.0.0:3000"))
    .run(app)
    .await
}




#[derive(Serialize, Deserialize, Debug)]
struct ErrorResponse {
  error: String,
  msg: String,
}