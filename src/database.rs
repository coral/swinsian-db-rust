use rusqlite::{params, Connection, Error as RusqliteError, OpenFlags, Result as DBResult};
use serde::{Deserialize, Serialize};
use serde_rusqlite::{
    columns_from_statement, from_row, from_row_with_columns, from_rows, Error as SerdeRusqliteError,
};
use std::collections::HashMap;
use std::{path::Path, slice::SliceIndex};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Playlist {
    playlist_id: i64,
    name: String,
    pindex: i64,
    applescriptid: String,
    smart: Option<i64>,
    smartpredicate: Option<Vec<u8>>,
    sortkey: Option<String>,
    ascending: i64,
    folder: Option<i64>,
    expanded: Option<i64>,
    itunes_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Track {
    track_id: i64,
    title: String,
    artist: String,
    album: String,
    genre: String,
    composer: String,
    year: Option<i64>,
    tracknumber: Option<i64>,
    discnumber: Option<i64>,
    bitrate: Option<i64>,
    bitdepth: Option<i64>,
    samplerate: Option<i64>,
    channels: Option<i64>,
    length: Option<f64>,
    dateadded: Option<f64>,
    lastplayed: Option<f64>,
    playcount: i64,
    rating: i64,
    filesize: i64,
    enabled: i64,
    cue: Option<i64>,
    gapless: Option<i64>,
    compilation: Option<i64>,
    encoder: String,
    path: String,
    filename: String,
    comment: String,
    properties_id: i64,
    albumartist: String,
    totaldiscnumber: Option<i64>,
    datecreated: Option<f64>,
    grouping: String,
    bpm: Option<i64>,
    publisher: String,
    totaltracknumber: Option<i64>,
    description: String,
    datemodified: f64,
    catalognumber: String,
    conductor: String,
    discsubtitle: String,
    lyrics: String,
    copyright: String,
}

impl Playlist {
    pub fn songs(&self) {}
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn from_file(filename: &Path) -> Result<Database, DatabaseError> {
        let db = Connection::open_with_flags(
            &filename,
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_NO_MUTEX
                | OpenFlags::SQLITE_OPEN_URI
                | OpenFlags::SQLITE_OPEN_EXRESCODE,
        )?;

        Ok(Database { conn: db })
    }

    pub fn get_playlist(&self, name: &str) -> Result<Playlist, DatabaseError> {
        let mut statement = self.conn.prepare("SELECT * FROM playlist where NAME = ?")?;

        let mut res = from_rows::<Playlist>(statement.query([name])?);

        let p = match res.next() {
            Some(p) => p,
            None => return Err(DatabaseError::PlaylistNotFound(name.to_string())),
        };

        Ok(p?)
    }

    pub fn get_playlists(&self) -> Result<HashMap<i64, Playlist>, DatabaseError> {
        let mut statement = self.conn.prepare("SELECT * FROM playlist")?;
        let res = from_rows::<Playlist>(statement.query([])?);

        let up: Result<_, _> = res
            .into_iter()
            .map(|p| p.map(|e| (e.playlist_id, e)))
            .collect();

        Ok(up?)
    }

    pub fn get_playlist_songs(&self, p: &Playlist) -> Result<Vec<Track>, DatabaseError> {
        let mut statement = self.conn.prepare(
            "SELECT * FROM track WHERE track_id = (SELECT  track_id FROM playlist WHERE playlist_id = ?)",
        )?;

        let res = from_rows::<Track>(statement.query([p.playlist_id])?);
        let tracks: Result<_, _> = res
            .into_iter()
            .map(|r| {
                r.map(|rm| {
                    dbg!(&rm);
                    rm
                })
            })
            .collect();

        Ok(tracks?)
    }

    //pub fn get_songs_for_playlist()
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
