mod bank;
mod tui;

pub use bank::*;

use banksystem_rs::tui::BankTui;

fn main() {
    if let Err(err) = BankTui::run() {
        eprintln!("Error: {}", err);
    }
}
