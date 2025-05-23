use crate::bank::Bank;
use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use tui::widgets::{Block, Borders, Paragraph, Clear};
use tui::layout::{Layout, Constraint, Direction, Rect};
use tui::style::{Style, Color as TuiColor};
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, ClearType};
use crossterm::{execute, terminal};
use crossterm::cursor::MoveTo;
use crossterm::style::{SetForegroundColor, Color, SetAttributes, Attribute};
use tui::text::{Span, Spans};

enum InputMode {
    MainMenu,
    AccountMenu,
    CreatingAccount,
    Deposit,
    Withdraw,
    Transfer,
}

pub fn run() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    
    // Only clear screen and set foreground color
    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        SetAttributes(Attribute::Reset.into()),
        MoveTo(0, 0),
        SetForegroundColor(Color::Green)
    )?;

    drop(stdout);
    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut bank = Bank::new();
    let mut input_mode = InputMode::MainMenu;
    let mut input = String::new();
    let mut message = String::new();
    let mut command = String::new();
    let mut running = true;

    while running {
        terminal.draw(|f| {
            let size = f.size();
            
            // Main layout
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints([
                    Constraint::Length(5),  // System info
                    Constraint::Min(10),    // Main content
                    Constraint::Length(3),  // Command/Message area
                    Constraint::Length(2),  // Function keys
                ].as_ref())
                .split(size);

            // System information header
            let system_info = vec![
                Spans::from(vec![
                    Span::styled("System     . . . . :", Style::default().fg(TuiColor::Green)),
                    Span::styled(" BANKSYSTEM-RS", Style::default().fg(TuiColor::Green))
                ]),
                Spans::from(vec![
                    Span::styled("Subsystem  . . . . :", Style::default().fg(TuiColor::Green)),
                    Span::styled(" BANKING", Style::default().fg(TuiColor::Green))
                ]),
                Spans::from(vec![
                    Span::styled("Display    . . . . :", Style::default().fg(TuiColor::Green)),
                    Span::styled(" MAIN", Style::default().fg(TuiColor::Green))
                ]),
            ];
            let system_info = Paragraph::new(system_info)
                .style(Style::default());
            f.render_widget(system_info, chunks[0]);

            // Main content area
            match input_mode {
                InputMode::MainMenu => {
                    let menu_items = vec![
                        "1. Account Management",
                        "2. Transaction History",
                        "3. System Settings",
                        "",
                        "90. Sign off"
                    ];
                    let menu = Paragraph::new(menu_items.join("\n"))
                        .style(Style::default().fg(TuiColor::Green));
                    f.render_widget(menu, chunks[1]);
                },
                InputMode::AccountMenu => {
                    let mut content = vec!["Select one of the following:".to_string(), "".to_string()];
                    for (i, acc) in bank.list_accounts().iter().enumerate() {
                        content.push(format!("{}. {} . . . . . . . . ${:.2}", 
                            i + 1, acc.name, acc.balance));
                    }
                    let accounts = Paragraph::new(content.join("\n"))
                        .style(Style::default().fg(TuiColor::Green));
                    f.render_widget(accounts, chunks[1]);
                },
                _ => {
                    let prompt = match input_mode {
                        InputMode::CreatingAccount => "Enter account name:",
                        InputMode::Deposit => "Enter deposit amount:",
                        InputMode::Withdraw => "Enter withdrawal amount:",
                        InputMode::Transfer => "Enter target account number,amount:",
                        _ => ""
                    };
                    let input_area = centered_rect(60, 8, chunks[1]);
                    let input_text = format!("{}\n{}", prompt, format!("{}_", input));
                    let popup = Paragraph::new(input_text)
                        .block(Block::default().borders(Borders::ALL))
                        .style(Style::default().fg(TuiColor::Green));
                    f.render_widget(Clear, input_area);
                    f.render_widget(popup, input_area);
                }
            }

            // Command/Message area
            let cmd_text = if !message.is_empty() {
                format!("{}\nSelection or command\n===> {}_", message, command)
            } else {
                format!("Selection or command\n===> {}_", command)
            };
            let cmd_area = Paragraph::new(cmd_text)
                .style(Style::default().fg(TuiColor::Green));
            f.render_widget(cmd_area, chunks[2]);

            // Function keys equivalent commands
            let commands = match input_mode {
                InputMode::MainMenu => "Commands: X=Exit",
                InputMode::AccountMenu => "Commands: X=Exit  C=Create  D=Deposit  W=Withdraw  T=Transfer",
                _ => "Commands: X=Cancel  P=Previous"
            };
            let keys = Paragraph::new(commands)
                .style(Style::default().fg(TuiColor::Green));
            f.render_widget(keys, chunks[3]);
        })?;

        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match input_mode {
                    InputMode::MainMenu => match key.code {
                        KeyCode::Enter => {
                            match command.trim().to_uppercase().as_str() {
                                "1" => {
                                    input_mode = InputMode::AccountMenu;
                                    command.clear();
                                },
                                "90" | "X" => running = false,
                                _ => {}
                            }
                        },
                        KeyCode::Char(c) => command.push(c),
                        KeyCode::Backspace => { command.pop(); },
                        _ => {}
                    },
                    InputMode::AccountMenu => match key.code {
                        KeyCode::Enter => {
                            match command.trim().to_uppercase().as_str() {
                                "X" => {
                                    input_mode = InputMode::MainMenu;
                                    command.clear();
                                },
                                "C" => {
                                    input_mode = InputMode::CreatingAccount;
                                    input.clear();
                                },
                                "D" => if !bank.accounts.is_empty() {
                                    input_mode = InputMode::Deposit;
                                    input.clear();
                                },
                                "W" => if !bank.accounts.is_empty() {
                                    input_mode = InputMode::Withdraw;
                                    input.clear();
                                },
                                "T" => if !bank.accounts.is_empty() {
                                    input_mode = InputMode::Transfer;
                                    input.clear();
                                },
                                num => if let Ok(n) = num.parse::<usize>() {
                                    if n > 0 && n <= bank.accounts.len() {
                                        message = format!("Selected account: {}", bank.accounts[n-1].name);
                                    }
                                }
                            }
                            if command.trim().to_uppercase() != "X" {
                                command.clear();
                            }
                        },
                        KeyCode::Char(c) => command.push(c),
                        KeyCode::Backspace => { command.pop(); },
                        _ => {}
                    },
                    _ => match key.code {
                        KeyCode::Enter => {
                            match input_mode {
                                InputMode::CreatingAccount => {
                                    if !input.trim().is_empty() {
                                        bank.create_account(input.trim().to_string());
                                        message = format!("Account '{}' created.", input.trim());
                                    }
                                },
                                InputMode::Deposit => {
                                    if let Ok(amount) = input.trim().parse::<f64>() {
                                        if let Some(acc) = bank.accounts.first_mut() {
                                            acc.deposit(amount);
                                            message = format!("Deposited ${:.2} to {}.", amount, acc.name);
                                        }
                                    }
                                },
                                InputMode::Withdraw => {
                                    if let Ok(amount) = input.trim().parse::<f64>() {
                                        if let Some(acc) = bank.accounts.first_mut() {
                                            if acc.withdraw(amount) {
                                                message = format!("Withdrew ${:.2} from {}.", amount, acc.name);
                                            } else {
                                                message = "Insufficient funds.".to_string();
                                            }
                                        }
                                    }
                                },
                                InputMode::Transfer => {
                                    let parts: Vec<&str> = input.trim().split(',').collect();
                                    if parts.len() == 2 {
                                        if let (Ok(target_num), Ok(amount)) = (parts[0].parse::<usize>(), parts[1].parse::<f64>()) {
                                            if target_num > 0 && target_num <= bank.accounts.len() {
                                                let source_index = 0;
                                                let target_index = target_num - 1;
                                                if source_index != target_index {
                                                    let accounts = &mut bank.accounts;
                                                    if source_index < target_index {
                                                        let (left, right) = accounts.split_at_mut(target_index);
                                                        let from_acc = &mut left[source_index];
                                                        let to_acc = &mut right[0];
                                                        if from_acc.transfer(amount, to_acc) {
                                                            message = format!("Transferred ${:.2} to account #{}.", amount, target_num);
                                                        } else {
                                                            message = "Insufficient funds.".to_string();
                                                        }
                                                    } else {
                                                        let (left, right) = accounts.split_at_mut(source_index);
                                                        let to_acc = &mut left[target_index];
                                                        let from_acc = &mut right[0];
                                                        if from_acc.transfer(amount, to_acc) {
                                                            message = format!("Transferred ${:.2} to account #{}.", amount, target_num);
                                                        } else {
                                                            message = "Insufficient funds.".to_string();
                                                        }
                                                    }
                                                } else {
                                                    message = "Cannot transfer to the same account.".to_string();
                                                }
                                            }
                                        }
                                    }
                                },
                                _ => {}
                            }
                            input_mode = InputMode::AccountMenu;
                            input.clear();
                        },
                        KeyCode::Char(c) => {
                            match (c.to_uppercase().to_string().as_str(), input.is_empty()) {
                                ("X", true) => {
                                    input_mode = InputMode::AccountMenu;
                                    input.clear();
                                },
                                ("P", true) => {
                                    input_mode = InputMode::AccountMenu;
                                    input.clear();
                                },
                                _ => input.push(c)
                            }
                        },
                        KeyCode::Backspace => { input.pop(); },
                        _ => {}
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        SetAttributes(Attribute::Reset.into()),
        terminal::Clear(ClearType::All),
        MoveTo(0, 0)
    )?;
    Ok(())
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
