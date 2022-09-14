# Text-based Reversi written in Rust

This project has nix support with the help of `cargo2nix`. To get started, type the following to set up the environment (nix flakes must be enabled):
```
nix develop
```

Other commands:
```
nix build .#reversi-rust
nix run .#reversi-rust
```

Or, using Rust's own tools:
```
cargo test
cargo run
```
