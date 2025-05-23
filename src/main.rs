mod bank;
mod tui;

pub use bank::*;

fn main() {
    tui::run().unwrap();
}
