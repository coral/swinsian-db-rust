use rusqlite::{Connection, Error as RusqliteError, OpenFlags};
use serde::{Deserialize, Serialize};
use serde_rusqlite::{from_rows, Error as SerdeRusqliteError};
use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Playlist {
    pub playlist_id: i64,
    pub name: String,
    pub pindex: i64,
    pub applescriptid: String,
    pub smart: Option<i64>,
    pub smartpredicate: Option<Vec<u8>>,
    pub sortkey: Option<String>,
    pub ascending: i64,
    pub expanded: Option<i64>,
    pub itunes_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Track {
    pub track_id: i64,
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub genre: Option<String>,
    pub composer: Option<String>,
    pub year: Option<i64>,
    #[serde(rename = "tracknumber")]
    pub track_number: Option<i64>,
    #[serde(rename = "discnumber")]
    pub disc_number: Option<i64>,
    pub bitrate: Option<i64>,
    #[serde(rename = "bitdepth")]
    pub bit_depth: Option<i64>,
    pub samplerate: Option<i64>,
    pub channels: Option<i64>,
    pub length: Option<f64>,
    #[serde(rename = "dateadded")]
    pub date_added: Option<f64>,
    #[serde(rename = "lastplayed")]
    pub last_played: Option<f64>,
    pub playcount: i64,
    pub rating: f64,
    pub filesize: i64,
    pub enabled: i64,
    pub cue: Option<i64>,
    pub gapless: Option<i64>,
    pub compilation: Option<i64>,
    pub encoder: Option<String>,
    pub path: String,
    pub filename: String,
    pub comment: Option<String>,
    pub properties_id: i64,
    #[serde(rename = "albumartist")]
    pub album_artist: Option<String>,
    #[serde(rename = "totaldiscnumber")]
    pub total_disc_number: Option<i64>,
    #[serde(rename = "datecreated")]
    pub date_created: Option<f64>,
    pub grouping: Option<String>,
    pub bpm: Option<i64>,
    pub publisher: Option<String>,
    #[serde(rename = "totaltracknumber")]
    pub total_track_number: Option<i64>,
    pub description: Option<String>,
    #[serde(rename = "datemodified")]
    pub date_modified: f64,
    #[serde(rename = "catalognumber")]
    pub catalog_number: Option<String>,
    pub conductor: Option<String>,
    #[serde(rename = "discsubtitle")]
    pub disc_subtitle: Option<String>,
    pub lyrics: Option<String>,
    pub copyright: Option<String>,
}

impl Playlist {
    //HAHA OK LOL NSPredicate IS NOT THAT EASY TO IMPLEMENT

    // pub fn get_predicate(&self) -> Result<(), DatabaseError> {
    //     self.smart.ok_or(DatabaseError::PlaylistNotSmart(self.name));

    //     let pred = self
    //         .smartpredicate
    //         .ok_or(DatabaseError::NoPredicate(self.name))?;
    //     plist::from_bytes(&pred);

    //     Ok(())
    // }
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn from_file(filename: &Path) -> Result<Database, DatabaseError> {
        let db = Connection::open_with_flags(
            &filename,
            OpenFlags::SQLITE_OPEN_READ_ONLY
                | OpenFlags::SQLITE_OPEN_NO_MUTEX
                | OpenFlags::SQLITE_OPEN_URI
                | OpenFlags::SQLITE_OPEN_EXRESCODE,
        )?;

        Ok(Database { conn: db })
    }

    pub fn get_playlist(&self, name: &str) -> Result<Playlist, DatabaseError> {
        let mut statement = self
            .conn
            .prepare("SELECT * FROM playlist where NAME = ? AND folder IS NULL")?;

        let mut res = from_rows::<Playlist>(statement.query([name])?);

        let p = match res.next() {
            Some(p) => p,
            None => return Err(DatabaseError::PlaylistNotFound(name.to_string())),
        };

        Ok(p?)
    }

    pub fn get_playlists(&self) -> Result<HashMap<i64, Playlist>, DatabaseError> {
        let mut statement = self
            .conn
            .prepare("SELECT * FROM playlist WHERE folder IS NULL")?;
        let res = from_rows::<Playlist>(statement.query([])?);

        let up: Result<_, _> = res
            .into_iter()
            .map(|p| p.map(|e| (e.playlist_id, e)))
            .collect();

        Ok(up?)
    }

    pub fn get_playlist_songs(&self, p: &Playlist) -> Result<Vec<Track>, DatabaseError> {
        let mut statement = self.conn.prepare(
            "SELECT * FROM track WHERE track_id IN (SELECT track_id FROM playlisttrack WHERE playlist_id = ?)",
        )?;

        let res = from_rows::<Track>(statement.query([p.playlist_id])?);
        let tracks: Result<_, _> = res.into_iter().map(|r| r.map(|rm| rm)).collect();

        Ok(tracks?)
    }
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error(transparent)]
    SQLiteError(#[from] RusqliteError),

    #[error(transparent)]
    SerdeSQLiteError(#[from] SerdeRusqliteError),

    #[error("Playlist not found {0}")]
    PlaylistNotFound(String),

    #[error("unknown data store error")]
    Unknown,
}
