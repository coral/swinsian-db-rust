use rusqlite::{params, Connection, Error as RusqliteError, Result as DBResult};
use std::path::Path;
use thiserror::Error;

mod playlist;

pub use playlist::Playlist;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn from_file(filename: &Path) -> Result<Database, DatabaseError> {
        let db = Connection::open(&filename)?;

        Ok(Database { conn: db })
    }
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error(transparent)]
    DBError(#[from] RusqliteError),

    #[error("unknown data store error")]
    Unknown,
}
