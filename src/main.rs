use std::error::Error;
use std::io::IsTerminal;
use clap::{Parser, Subcommand};

mod bundle;
mod string_utils;

use bundle::Bundle;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Mermaid {
        #[arg(long)]
        url: bool,
    },
    Graphviz {},
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::io::stdin();

    let bundle: Bundle = if input.is_terminal() {
        // Empty stdin - try to read "bundle.yaml"
        let f = std::fs::File::open("bundle.yaml")?;
        serde_yaml::from_reader(f)?
    } else {
        // Read from stdin
        serde_yaml::from_reader(input.lock())?
    };

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Mermaid { url}) => {
            if *url {
                // println!("{}", bundle.to_edit_url());
                println!("{}", bundle.to_img_url());
            } else {
                println!("{}", bundle.to_mermaid());
            }
        }
        Some(Commands::Graphviz {} ) => {
            println!("{}", bundle.to_graphviz());
        }
        None => {}
    }

    Ok(())
}
