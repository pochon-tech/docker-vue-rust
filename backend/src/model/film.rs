use serde::{Deserialize, Serialize};
use deadpool_postgres::{Client, PoolError};

#[derive(Deserialize, Serialize)]
pub struct Film {
  pub film_id: i32,
  pub title: String,
}

impl Film {
  pub async fn list(client: &Client) -> Result<Vec<Film>, PoolError> {
    let stmt = client.prepare(r#"
      SELECT
        film_id
        ,title
      FROM
        film
    "#).await?;
    let rows = client.query(&stmt, &[]).await?;
    Ok(rows
      .into_iter()
      .map(|row| Film {
        film_id: row.get(0),
        title: row.get(1),
      })
      .collect()
    )
  }
}
