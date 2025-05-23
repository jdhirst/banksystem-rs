use tui::widgets::ListState;

pub struct BankTui {
    pub state: ListState,
    current_screen: Screen,
    pub bank: Bank,
}

impl BankTui {
    pub fn new(bank: Bank) -> Self {
        Self {
            state: ListState::default(),
            current_screen: Screen::MainMenu,
            bank,
        }
    }

    pub fn get_screen(&self) -> &Screen {
        &self.current_screen
    }

    pub fn set_screen(&mut self, screen: Screen) {
        self.current_screen = screen;
        self.state = ListState::default();
    }
}

#[derive(PartialEq)]
pub enum Screen {
    MainMenu,
    CustomerList,
    NewCustomer(Form),
    NewAccount(Form),
    AccountList,
    AccountDetails(u64),
    Transfer,
}

mod renderer {
    use super::*;
    use tui::{
        text::{Spans, Span},
        style::{Style, Color},
        widgets::{Block, Borders, List, ListItem},
    };

    pub fn render_customer_list<'a>(bank: &'a Bank) -> List<'a> {
        let customers = bank.list_customers();
        let items: Vec<ListItem> = customers
            .iter()
            .map(|c| {
                ListItem::new(Spans::from(vec![
                    Span::raw(format!("ID: {} - ", c.id)),
                    Span::styled(&c.name, Style::default().fg(Color::Green)),
                ]))
            })
            .collect();

        List::new(items)
            .block(Block::default().title("Customers").borders(Borders::ALL))
            .highlight_style(Style::default().fg(Color::Yellow))
    }

    pub fn render_account_list<'a>(bank: &'a Bank) -> List<'a> {
        let accounts = bank.list_accounts();
        let items: Vec<ListItem> = accounts
            .iter()
            .map(|a| {
                ListItem::new(Spans::from(vec![
                    Span::raw(format!("ID: {} - ", a.id)),
                    Span::styled(&a.name, Style::default().fg(Color::Green)),
                    Span::raw(format!(" ({})", a.balance)),
                ]))
            })
            .collect();

        List::new(items)
            .block(Block::default().title("Accounts").borders(Borders::ALL))
            .highlight_style(Style::default().fg(Color::Yellow))
    }

    pub fn render_account_details<'a>(bank: &'a Bank, id: u64) -> Paragraph<'a> {
        if let Some(account) = bank.get_account(id) {
            let customer = bank.get_customer(account.customer_id)
                .map(|c| c.name.as_str())
                .unwrap_or("Unknown");

            let history: Vec<String> = account.history
                .iter()
                .map(|t| format!("{:?}: ${:.2}", t.transaction_type, t.amount))
                .collect();

            let content = format!(
                "Account ID: {}\nCustomer: {}\nType: {}\nBalance: ${:.2}\n\nTransaction History:\n{}",
                account.id,
                customer,
                account.account_type,
                account.balance,
                history.join("\n")
            );

            Paragraph::new(content)
                .block(Block::default().title("Account Details").borders(Borders::ALL))
                .style(Style::default().fg(Color::Green))
        } else {
            Paragraph::new("Account not found")
                .block(Block::default().title("Error").borders(Borders::ALL))
                .style(Style::default().fg(Color::Red))
        }
    }

    pub fn render_transfer_form<'a>() -> Paragraph<'a> {
        let content = "Transfer funds form\n\nFrom Account: [    ]\nTo Account:   [    ]\nAmount:      [    ]";
        
        Paragraph::new(content)
            .block(Block::default().title("Transfer").borders(Borders::ALL))
            .style(Style::default().fg(Color::Green))
    }
}

use renderer::*;
