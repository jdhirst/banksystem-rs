use crate::bank::{Bank, Account};
use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Style, Color};
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

pub fn run() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut bank = Bank::new();
    let mut selected = 0;
    let mut running = true;
    while running {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().title("IBM Bank System").borders(Borders::ALL);
            f.render_widget(block, size);
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(5),
                ].as_ref())
                .split(size);
            let accounts: Vec<ListItem> = bank.list_accounts().iter().map(|a| {
                ListItem::new(format!("#{}: {} | ${:.2}", a.id, a.name, a.balance))
            }).collect();
            let list = List::new(accounts)
                .block(Block::default().borders(Borders::ALL).title("Accounts"))
                .highlight_style(Style::default().bg(Color::Blue));
            f.render_widget(list, chunks[1]);
            let help = Paragraph::new("N: New | D: Deposit | W: Withdraw | T: Transfer | Q: Quit");
            f.render_widget(help, chunks[0]);
        })?;
        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => running = false,
                    // Add more key handling here for N, D, W, T
                    _ => {}
                }
            }
        }
    }
    disable_raw_mode()?;
    Ok(())
}
