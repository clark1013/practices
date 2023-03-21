use std::collections::HashMap;

use clap::Parser;
use config::FileFormat;
use serde::Deserialize;
use toydb::error::Result;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    config_file: String,
    #[arg(short, long)]
    id: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let config = Config::new(&args.config_file)?;
    let raft_listen = config.clusters.get(&args.id).unwrap().clone();
    let mut peers = config.clusters;
    peers.remove(&args.id);
    toydb::server::Server::new(&args.id, peers)
        .await?
        .listen(&raft_listen)
        .await?
        .serve()
        .await
}

#[derive(Debug, Deserialize)]
struct Config {
    clusters: HashMap<String, String>,
}

impl Config {
    pub fn new(file: &str) -> Result<Self> {
        let builder = config::Config::builder()
            .set_default("clusters", HashMap::<String, String>::new())?
            .add_source(config::File::new(file, FileFormat::Yaml));
        let c = builder.build()?;
        Ok(c.try_deserialize()?)
    }
}
