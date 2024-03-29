pub mod model;
pub mod query;

use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use sqlx::Sqlite;
use uuid::Uuid;
use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
pub enum DataError {
  #[error("database error: {0}")]
  Database(#[from] sqlx::Error),
}

pub type AppDatabase = Database<Sqlite>;
pub type DatabasePool = sqlx::sqlite::SqlitePool;
pub type Transaction<'t> = sqlx::Transaction<'t, Sqlite>;
pub type AppDatabaseRow = sqlx::sqlite::SqliteRow;
pub type AppQueryResult = sqlx::sqlite::SqliteQueryResult;


pub struct Database<D: sqlx::Database>(sqlx::Pool<D>);

impl Database<Sqlite> {
  pub async fn new(connection_string: &str) -> Self {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
      .connect(connection_string)
      .await;

    match pool {
      Ok(pool) => Self(pool),
      Err(e) => {
        eprintln!("{}\n", e);
        eprintln!("If database ha not yet been created, run: \n $ sqlx database setup\n");
        panic!("database error");
      }
    }
  }

  pub fn get_pool(&self) -> &DatabasePool {
    &self.0
  }
}


#[derive(Clone, Debug, Display, From, Deserialize, Serialize)]
pub struct DbId(Uuid);

impl DbId {
  pub fn new() -> DbId {
    Uuid::new_v4().into()
  }

  pub fn nil() -> DbId {
    Self(Uuid::nil())
  }
}

impl Default for DbId {
  fn default() -> Self {
    Self::new()
  }
}

impl FromStr for DbId {
  type Err = uuid::Error;

  fn from_str(id: &str) -> Result<Self, Self::Err> {
    Ok(DbId(Uuid::parse_str(id)?))
  }
}