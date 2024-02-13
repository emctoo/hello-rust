#![allow(unused)]

use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::types::chrono::NaiveDateTime;

// type PgTimetz = sqlx::postgres::types::PgTimeTz<NaiveDate, FixedOffset>

#[derive(sqlx::FromRow, Debug, Serialize)]
struct Xpi {
    date: NaiveDateTime,
    cpi: f64,
    ppi: f64,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let database_url = "postgres://postgres:postgres@127.0.0.1:6543/postgres";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    let sql = "select date, cpi, ppi from mi2_xpi order by date desc limit 3";
    let rows: Vec<Xpi> = sqlx::query_as(sql).fetch_all(&pool).await?;
    for row in rows {
        // println!("row: {:?}", serde_json::to_string(&row));
        println!("{:?}", row);
    }
    Ok(())
}
