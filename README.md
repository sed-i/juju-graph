# juju-graph

Convert a juju bundle to a mermaid diagram.

## Install
```bash
cargo install --git https://github.com/sed-i/juju-graph
```

## Usage
Convert bundle to mermaid:
```shell
juju export-bundle | juju-graph mermaid
```

Render a mermaid.live image url:
```shell
juju export-bundle | juju-graph mermaid --url
```

Render svg from graphviz:
```shell
juju export-bundle | juju-graph graphviz | dot -Tsvg > bundle.svg 
```


## Development
### Run from source

```bash
# Read from "bundle.yaml"
cargo run

# Pass via stdin
cargo run < bundle.yaml

# or
juju export-bundle | cargo run
```

### Install from source dir
```bash
cargo install --path .
juju-graph < bundle.yaml
```


## References
- https://github.com/mermaid-js/mermaid-live-editor/discussions/1291
