use clap::{Parser, Subcommand};
use env_logger::Env;
use kvs::{Client, KvStore, KvsEngine, Result};
use log::info;

#[derive(Parser, Debug)]
#[command(version)]
struct Arg {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(long, default_value = "localhost:4000")]
    addr: String,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Get {
        key: Option<String>,
    },
    Rm {
        key: Option<String>,
    },
    Set {
        key: Option<String>,
        value: Option<String>,
    },
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    let arg = Arg::parse();
    // info!("addr {}", &arg.addr);
    let mut client = Client::new(arg.addr);
    match arg.command {
        Some(Commands::Get { key }) => {
            client.get(key.unwrap())?;
        }
        Some(Commands::Set { key, value }) => {
            client.set(key.unwrap(), value.unwrap())?;
        }
        Some(Commands::Rm { key }) => {
            client.remove(key.unwrap())?;
        }
        None => panic!(),
    }
    Ok(())
}
