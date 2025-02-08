use poem::{get, handler, listener::TcpListener, web::Path, Route, Server};


mod user;

#[handler]
fn hello(Path(name): Path<String>) -> String {
  format!("hello: {}", name)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
  let app = Route::new()
    .at("/hello/:name", get(hello))
    .nest("/user", user::route())
    ;

  // user::init(&app);


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


