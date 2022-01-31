use clap::Parser;
use std::path::Path;
use swinsiandb::Database;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    database_path: String,
}

fn main() {
    let args = Args::parse();

    let db_path = Path::new(&args.database_path);
    Database::from_file(db_path).unwrap();

    println!("Hello, world!");
}
