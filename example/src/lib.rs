use std::net::SocketAddr;
use axum::Server;

mod router;
mod config;
mod graphql;
mod state;

pub use config::CONFIG;
use state::State;
use router::router;

pub async fn start() {
  let state = State;

  let app = router(state);
  let addr = SocketAddr::from(([0, 0, 0, 0], CONFIG.port));

  println!("server listening on http://127.0.0.1:{}", CONFIG.port);

  Server::bind(&addr)
      .serve(app.into_make_service())
      .await
      .unwrap();
}
