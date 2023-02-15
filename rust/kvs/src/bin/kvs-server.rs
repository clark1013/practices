use std::{env::current_dir, thread::panicking};

use clap::Parser;
use env_logger::Env;
use kvs::{KvStore, KvsServer, Result, SledStore};
use log::info;

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(long, default_value = "localhost:4000")]
    addr: String,
    #[arg(long, default_value = "kvs")]
    engine: String,
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    let args = Args::parse();
    info!("kvs-server {}", env!("CARGO_PKG_VERSION"));
    info!("storage engine {}", args.engine);
    info!("address {}", args.addr);
    if (args.engine == "kvs") {
        let mut server = KvsServer::new(args.addr, KvStore::open(current_dir()?)?);
        server.run()?;
    } else if args.engine == "sled" {
        let mut server = KvsServer::new(args.addr, SledStore::open(current_dir()?)?);
        server.run()?;
    } else {
        panic!("")
    }
    Ok(())
}
