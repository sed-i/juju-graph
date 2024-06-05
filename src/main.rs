use crate::petgraph_wrappers::GraphAsCode;
use clap::{Parser, Subcommand};
use std::{io::IsTerminal, process};

mod bundle;
mod errors;
mod petgraph_wrappers;
mod string_utils;

use crate::{
    errors::{Error, Result},
    string_utils::MermaidRelated,
};
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
        #[arg(long, default_value = "")]
        spotlight: String,
    },
    Graphviz {
        #[arg(long, default_value = "")]
        spotlight: String,
    },
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let input = std::io::stdin();

    let bundle: Bundle = if input.is_terminal() {
        // Empty stdin - try to read "bundle.yaml"
        let f = std::fs::File::open("bundle.yaml").map_err(|cause| Error::IO {
            path: "bundle.yaml".into(),
            cause,
        })?;
        serde_yaml::from_reader(f)?
    } else {
        // Read from stdin
        serde_yaml::from_reader(input.lock())?
    };

    let cli = Cli::parse();

    let graph = bundle.to_graph();

    match &cli.command {
        Some(Commands::Mermaid { url, spotlight }) => {
            let graph = if spotlight.is_empty() {
                graph
            } else {
                graph.neighbors(spotlight)
            };

            if *url {
                // println!("{}", bundle.to_edit_url());
                println!("{}", graph.graph.to_mermaid().to_img_url());
            } else {
                println!("{}", graph.graph.to_mermaid());
            }
        }
        Some(Commands::Graphviz { spotlight }) => {
            let graph = if spotlight.is_empty() {
                graph
            } else {
                graph.neighbors(spotlight)
            };
            println!("{}", graph.graph.to_graphviz());
        }
        None => {
            println!("Use --help for usage details.");
        }
    }

    Ok(())
}
