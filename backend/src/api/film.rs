use actix_web::{get, HttpResponse, web};
use deadpool_postgres::{Client, Pool};
use crate::{
  model::film::Film,
  error::Error
};

#[get("/api/films")]
async fn index(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
  let client: Client = db_pool.get().await?;
  let films = Film::list(&client).await?;
  Ok(HttpResponse::Ok().json(films))
}
