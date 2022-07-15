pub mod core;
pub mod backend;
pub mod frontend;
pub mod components;
pub mod devices;

fn main() {
    println!("crossemu - Version: {}", env!("CARGO_PKG_VERSION"));
}
