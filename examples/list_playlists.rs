use anyhow::Result;
use clap::Parser;
use std::path::Path;
use swinsiandb::Database;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    database_path: String,
}

fn main() -> Result<()> {
    //Parse path to Swinsian DB
    //Usually located at /Users/<YOU>/Library/Application Support/Swinsian/Library.sqlite
    let args = Args::parse();
    let db_path = Path::new(&args.database_path);

    //Load it up
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
        println!("{} - {}", song.title, song.artist.unwrap_or_default());
    }

    Ok(())
}
