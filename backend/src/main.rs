use actix_web::{App, HttpServer};
use dotenv::dotenv;
use tokio_postgres::NoTls;

mod api;
mod config;
mod error;
mod model;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();

  let config = crate::config::Config::from_env().unwrap();
  let pool = config.pg.create_pool(NoTls).unwrap();

  HttpServer::new(move || {
    App::new()
      .data(pool.clone())
      .service(api::film::index)
  })
  .bind(config.server_addr.clone())?
  .run()
  .await
}
