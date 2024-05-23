# juju-graph

Convert a juju bundle to a mermaid diagram.


## Run from source

```bash
# Read "bundle.yaml"
cargo run

# Pass as stdin
cargo run < bundle.yaml

# or
juju export-bundle | cargo run
```

## Install from source dir
```bash
cargo install --path .
juju-graph < bundle.yaml
```

## Install via repo url
```bash
cargo install --git https://github.com/sed-i/juju-graph
```

## References
- https://github.com/mermaid-js/mermaid-live-editor/discussions/1291
