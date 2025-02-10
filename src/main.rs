use play_asia::all_routes;
use poem::{listener::TcpListener, Server};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
  let routes = all_routes("data.json".to_string());

  Server::new(TcpListener::bind("0.0.0.0:3000"))
    .run(routes)
    .await
}