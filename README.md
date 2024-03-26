# Hooky
Git hooks helper made in Rust

## Features
- [x] Zero dependencies
- [x] Easy to use
- [x] Fast
- [x] Cross-platform

# Installation
```bash
cargo install hooky-rs
```

# CLI usage

## Initialize Hooky
To initialize the hooks, run the following command:
```bash
hooky init
```
This will create a `.hooky` directory in the root of your project, where the hooks will be stored. Also automatically adds `pre-commit` and hooks, but you can disable this by passing the `--no-pre-commit` flag.

## Add a hook
To add a hook, run the following command:
```bash
hooky add <hook-name>
```


## Uninstall Hooky
To uninstall Hooky, run the following command:
```bash
hooky uninstall
```

# Library usage (Experimental)
The library usage is made for Rust projects, to bind the hooks automatically.

To use Hooky as a library, add the following to your `Cargo.toml`:
```toml
[build-dependencies]
hooky-rs = "*"
```

Add `src/build.rs`:
```rust
fn main() {
    // Initialize Hooky
    hooky::initialize_build();
}
```