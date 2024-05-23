use std::error::Error;
use std::io::IsTerminal;

mod bundle;
mod string_utils;

use bundle::Bundle;

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

    //println!("{}", bundle.to_mermaid());
    println!("{}", bundle.to_graphviz());
    //println!("{}", bundle.to_img_url());
    //println!("{}", bundle.to_edit_url());

    Ok(())
}
