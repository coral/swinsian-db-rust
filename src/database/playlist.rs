use serde::{Deserialize, Serialize};
use serde_rusqlite::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Playlist {
    id: i64,
    name: String,
    pindex: i64,
    applescriptid: String,
    smart: i64,
    smartpredicate: Vec<u8>,
    sortkey: String,
    ascending: i64,
    folder: i64,
    expanded: i64,
    itunes_id: String,
}
