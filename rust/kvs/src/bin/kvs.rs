use clap::{Parser, Subcommand};

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

fn main() {
    let arg = Arg::parse();
    match arg.command {
        Some(Commands::Get { key }) => {
            panic!("unimplemented")
        }
        Some(Commands::Set { key, value }) => {
            panic!("unimplemented")
        }
        Some(Commands::Rm { key }) => {
            panic!("unimplemented")
        }
        None => panic!(),
    }
    // println!("{:?}", arg);
}
