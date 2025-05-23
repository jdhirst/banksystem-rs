use tui::layout::{Layout, Constraint, Direction, Rect};
use tui::style::{Style, Color};
use tui::widgets::{Block, Borders, Paragraph, Clear};
use tui::text::{Spans, Span};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use super::{BankTui, forms::{Form, FormField}};
use std::io;

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

pub fn run_tui(tui: &mut BankTui) -> io::Result<()> {
    enable_raw_mode()?;
    let mut terminal = setup_terminal()?;

    while let Ok(true) = handle_screen(&mut terminal, tui) {}

    disable_raw_mode()?;
    Ok(())
}

fn setup_terminal() -> io::Result<tui::Terminal<tui::backend::CrosstermBackend<io::Stdout>>> {
    let stdout = io::stdout();
    let backend = tui::backend::CrosstermBackend::new(stdout);
    tui::Terminal::new(backend)
}

fn create_customer_form() -> Form {
    Form::new("Create New Customer", vec![
        FormField::new("Name", true, 50),
        FormField::new("Address", true, 100),
        FormField::new("Phone", true, 20)
            .with_validation(Box::new(|s| s.chars().all(|c| c.is_ascii_digit() || c == '-'))),
        FormField::new("Email", true, 50)
            .with_validation(Box::new(|s| s.contains('@'))),
    ])
}

fn create_account_form() -> Form {
    Form::new("Create New Account", vec![
        FormField::new("Customer ID", true, 20)
            .with_validation(Box::new(|s| s.parse::<u64>().is_ok())),
        FormField::new("Account Type", true, 20),
    ])
}

fn handle_screen<B: tui::backend::Backend>(
    terminal: &mut tui::Terminal<B>,
    tui: &mut BankTui,
) -> io::Result<bool> {
    terminal.draw(|f| {
        let size = f.size();
        
        // Header
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Title
                Constraint::Min(10),    // Content
                Constraint::Length(3),  // Status
            ].as_ref())
            .split(size);

        let title = render_title(tui.get_screen());
        f.render_widget(title, chunks[0]);

        match tui.get_screen() {
            Screen::MainMenu => render_main_menu(f, chunks[1]),
            Screen::NewCustomer(form) => {
                let area = centered_rect(60, 40, chunks[1]);
                f.render_widget(Clear, area);
                form.render(f, area);
            },
            Screen::NewAccount(form) => {
                let area = centered_rect(60, 30, chunks[1]);
                f.render_widget(Clear, area);
                form.render(f, area);
            },
            // ... other screens
        }
    })?;

    if let Event::Key(key) = crossterm::event::read()? {
        match tui.get_screen() {
            Screen::MainMenu => match key.code {
                KeyCode::Char('1') => tui.set_screen(Screen::NewCustomer(create_customer_form())),
                KeyCode::Char('2') => tui.set_screen(Screen::CustomerList),
                KeyCode::Char('3') => tui.set_screen(Screen::NewAccount(create_account_form())),
                KeyCode::Char('4') => tui.set_screen(Screen::AccountList),
                KeyCode::Char('x') | KeyCode::Char('X') => return Ok(false),
                _ => {}
            },
            Screen::NewCustomer(ref mut form) | Screen::NewAccount(ref mut form) => {
                match key.code {
                    KeyCode::Tab => form.next_field(),
                    KeyCode::BackTab => form.prev_field(),
                    KeyCode::Char(c) => form.input(c),
                    KeyCode::Backspace => form.backspace(),
                    KeyCode::Enter => {
                        if form.is_valid() {
                            let values = form.get_values();
                            match tui.get_screen() {
                                Screen::NewCustomer(_) => {
                                    let bank = tui.get_bank_mut();
                                    bank.create_customer(
                                        values[0].clone(),
                                        values[1].clone(),
                                        values[2].clone(),
                                        values[3].clone(),
                                    );
                                },
                                Screen::NewAccount(_) => {
                                    if let Ok(customer_id) = values[0].parse::<u64>() {
                                        let bank = tui.get_bank_mut();
                                        bank.create_account(customer_id, values[1].clone());
                                    }
                                },
                                _ => unreachable!(),
                            }
                            tui.set_screen(Screen::MainMenu);
                        }
                    },
                    KeyCode::Esc => tui.set_screen(Screen::MainMenu),
                    _ => {}
                }
            },
            // ... other screens
        }
    }

    Ok(true)
}

fn render_title(screen: &Screen) -> Paragraph<'static> {
    let title = match screen {
        Screen::MainMenu => "BANKSYSTEM-RS Main Menu",
        Screen::NewCustomer(_) => "Create New Customer",
        Screen::NewAccount(_) => "Create New Account",
        Screen::CustomerList => "Customer List",
        Screen::AccountList => "Account List",
        Screen::AccountDetails(_) => "Account Details",
        Screen::Transfer => "Transfer Funds",
    };

    Paragraph::new(vec![
        Spans::from(vec![
            Span::styled(title, Style::default().fg(Color::Green))
        ]),
    ])
    .style(Style::default())
}

fn render_main_menu<B: tui::backend::Backend>(f: &mut tui::Frame<B>, area: Rect) {
    let menu_items = vec![
        "1. Create New Customer",
        "2. View Customers",
        "3. Create New Account",
        "4. View Accounts",
        "",
        "X. Exit System",
    ];

    let menu = Paragraph::new(menu_items.join("\n"))
        .style(Style::default().fg(Color::Green))
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(menu, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
