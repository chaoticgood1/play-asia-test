use poem::{get, handler, listener::TcpListener, web::Path, Route, Server};

mod user;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
  let app = Route::new()
    .nest("/user", user::route());

  Server::new(TcpListener::bind("0.0.0.0:3000"))
    .run(app)
    .await
}







/*
  TODO:
    Sign in
    Log in
    Log out
    Login
      user: Admin
      pass: Admin
    Logout

*/


