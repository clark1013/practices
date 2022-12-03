use clap::{Parser, Subcommand};
use kvs::{KvStore, Result};
use std::env::current_dir;

#[derive(Parser, Debug)]
#[command(version)]
struct Arg {
    #[command(subcommand)]
    command: Option<Commands>,
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
    let arg = Arg::parse();
    match arg.command {
        Some(Commands::Get { key }) => {
            let store = KvStore::open(current_dir()?);
            if let Some(value) = store?.get(key.unwrap())? {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
        }
        Some(Commands::Set { key, value }) => {
            let store = KvStore::open(current_dir()?);
            store?.set(key.unwrap(), value.unwrap())?;
        }
        Some(Commands::Rm { key }) => {
            let store = KvStore::open(current_dir()?);
            if store?.remove(key.unwrap()).is_err() {
                println!("Key not found");
                panic!()
            }
        }
        None => panic!(),
    }
    Ok(())
}
