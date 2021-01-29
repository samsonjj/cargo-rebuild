# cargo-rebuild
cargo-rebuild is a Cargo subcommand which forces a rebuild of the project.

### implementation
behind the scenes, it just calls `cargo clean` and then `cargo build`.

### installation
`cargo install cargo-rebuild`

### usage
`cargo rebuild`
