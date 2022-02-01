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
    let args = Args::parse();

    println!("Listing playlists");

    let db_path = Path::new(&args.database_path);
    let mut db = Database::from_file(db_path)?;

    let playlists = db.get_playlists();
    //dbg!(playlists);

    let d = db.get_playlist("TECHNO")?;

    let songs = db.get_playlist_songs(&d)?;

    //dbg!(songs);

    Ok(())
}
