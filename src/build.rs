#[path = "lib.rs"]
mod hooky;

fn main() {
    if option_env!("CARGO_PRIMARY_PACKAGE").is_some() {
        hooky::initialize_build()
    }
}
