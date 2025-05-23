mod forms;
mod widgets;
mod screens;

use crate::bank::Bank;
pub use forms::FormField;
pub use screens::Screen;

pub struct BankTui {
    bank: Bank,
    current_screen: Screen,
}

impl BankTui {
    pub fn new() -> Self {
        Self {
            bank: Bank::new(),
            current_screen: Screen::MainMenu,
        }
    }

    pub fn run() -> std::io::Result<()> {
        let mut tui = BankTui::new();
        screens::run_tui(&mut tui)
    }

    pub fn get_bank(&self) -> &Bank {
        &self.bank
    }

    pub fn get_bank_mut(&mut self) -> &mut Bank {
        &mut self.bank
    }

    pub fn set_screen(&mut self, screen: Screen) {
        self.current_screen = screen;
    }

    pub fn get_screen(&self) -> &Screen {
        &self.current_screen
    }
}
