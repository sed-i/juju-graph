use crate::petgraph_wrappers::GraphAsCode;
use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::io::IsTerminal;
use std::io::Read;
use std::io::Stdin;

mod bundle;
mod petgraph_wrappers;
mod string_utils;

use crate::string_utils::MermaidRelated;
use bundle::Bundle;

/// Transform juju bundles into diagrams:
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Output as mermaid diagram (see https://mermaid.live)
    Mermaid {
        /// Whether to render a mermaid.live image url
        #[arg(long)]
        url: bool,

        /// Output a subgraph containing only the given app and its immediate neighbors
        #[arg(long)]
        spotlight: Option<String>,
    },

    /// Output as graphviz diagram
    Graphviz {
        /// Output a subgraph containing only the given app and its immediate neighbors
        #[arg(long)]
        spotlight: Option<String>,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let mut input = std::io::stdin();

    let contents: String = if input.is_terminal() {
        // Empty stdin - try to read "bundle.yaml"
        // First reading to string because it's the only way to read a multi-doc yaml

        //let f = std::fs::File::open("bundle.yaml")?;
        fs::read_to_string("bundle.yaml")?
        //serde_yaml::from_reader(f)?
    } else {
        // Read from stdin
        let mut contents: String = String::new();
        Stdin::read_to_string(&mut input, &mut contents)?;
        contents
        //serde_yaml::from_reader(input.lock())?
    };

    let mut d = serde_yaml::Deserializer::from_str(&contents);
    let bundle: Bundle = loop {
        if let Some(dd) = d.next() {
            if let Ok(bundle) = Bundle::deserialize(dd) {
                break bundle;
            }
        } else {
            panic!("Invalid yaml");
        }
    };

    let graph = bundle.to_graph();

    match &cli.command {
        Commands::Mermaid { url, spotlight } => {
            let graph = if let Some(spotlight) = spotlight {
                graph.neighbors(spotlight)
            } else {
                graph
            };

            if *url {
                // println!("{}", bundle.to_edit_url());
                println!("{}", graph.graph.to_mermaid().to_img_url());
            } else {
                println!("{}", graph.graph.to_mermaid());
            }
        }
        Commands::Graphviz { spotlight } => {
            let graph = if let Some(spotlight) = spotlight {
                graph.neighbors(spotlight)
            } else {
                graph
            };
            println!("{}", graph.graph.to_graphviz());
        }
    }

    Ok(())
}
