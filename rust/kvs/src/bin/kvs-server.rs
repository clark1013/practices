use std::env::current_dir;

use clap::Parser;
use kvs::{KvStore, KvsServer, Result};

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(long, default_value = "localhost:4000")]
    addr: String,
    #[arg(long, default_value = "kvs")]
    engine: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let server = KvsServer::new(args.addr, KvStore::open(current_dir()?)?);
    server.run()?;
    Ok(())
}
