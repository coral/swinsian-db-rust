# Swinsian Rust Library Shim

My favorite OSX music player is by far [Swinsian](https://swinsian.com/). The API however ranges from non-existant to absolute trash (AppleScript). Lucky for the users, most of the app is backed by a SQLite database which we can read however we want.

This is just a simple wrapper around that database that allows one to pull playlists and songs from playlists.

```rust
//Load the Swinsian database
//Usually located at /Users/<YOU>/Library/Application Support/Swinsian/Library.sqlite
let db = Database::from_file(db_path)?;

//List all playlists
let playlists = db.get_playlists()?;
for (_, playlist) in playlists {
    print!("{}, ", playlist.name);
}

//Get songs from a playlist
let d = db.get_playlist("TECHNO")?;
let songs = db.get_playlist_songs(&d)?;
for song in songs {
    println!("{} - {}", song.title, ez(song.artist));
}
```
