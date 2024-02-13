use std::time::Duration;

use sea_orm::{ConnectOptions, Database, EntityTrait, QueryOrder};
use sea_orm::entity::prelude::*;

mod xpi {
    use chrono::NaiveDateTime;
    use sea_orm::{
        ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
        EnumIter, PrimaryKeyTrait,
    };
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "mi2_xpi", schema_name = "public")]
    pub struct Model {
        #[sea_orm(primary_key)]
        id: i32,
        date: NaiveDateTime,
        cpi: f64,
        ppi: f64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let database_url = "postgres://postgres:postgres@127.0.0.1:6543/postgres";
    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        // .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("public"); // Setting default PG schema
    let db = Database::connect(opt).await?;

    let xpi_list = xpi::Entity::find()
        .order_by_asc(xpi::Column::Date)
        .all(&db)
        .await?;
    // println!("xpi: {:?}", xpi);
    for xpi in xpi_list {
        println!("{:?}", xpi);
    }
    Ok(())
}