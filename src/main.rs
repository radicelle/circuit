mod app;
mod login_screen;
mod chat_screen;
mod irc_client;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, sync::Arc};
use tokio::sync::Mutex;
use ratatui::{
    prelude::*, Terminal
};
use crate::app::{App, InputField, InputMode};
use tokio::signal::ctrl_c;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let app = Arc::new(Mutex::new(App::default()));

    // Handle Ctrl+C
    let app_clone = Arc::clone(&app);
    tokio::spawn(async move {
        if let Ok(()) = ctrl_c().await {
            let mut app = app_clone.lock().await;
            app.connected = false;
            std::process::exit(0);  // Force exit the program
        }
    });

    let res = run_app(&mut terminal, app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: Arc<Mutex<App>>) -> io::Result<()> {
    loop {
        let app_state = app.lock().await;
        terminal.draw(|f| ui(f, &app_state))?;
        drop(app_state);

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                if key.code == KeyCode::Char('c') && key.modifiers == event::KeyModifiers::CONTROL {
                    return Ok(());
                }

                let mut app_lock = app.lock().await;
                match app_lock.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('e') => {
                            app_lock.input_mode = InputMode::Editing;
                        }
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        KeyCode::Tab => {
                            if !app_lock.connected {
                                app_lock.current_field = match app_lock.current_field {
                                    InputField::Nickname => InputField::Hostname,
                                    InputField::Hostname => InputField::Channel,
                                    InputField::Channel => InputField::Password,
                                    InputField::Password => InputField::Nickname,
                                    InputField::Message => InputField::Message,
                                };
                            }
                        }
                        KeyCode::Enter => {
                            let can_connect = !app_lock.connected
                                && app_lock.input_mode == InputMode::Editing
                                && matches!(app_lock.current_field, InputField::Password)
                                && !app_lock.nickname.is_empty()
                                && !app_lock.hostname.is_empty()
                                && !app_lock.channel.is_empty();

                            if can_connect {
                                drop(app_lock);
                                let app_clone = Arc::clone(&app);
                                if let Err(e) = irc_client::connect_to_server(app_clone).await {
                                    let mut app = app.lock().await;
                                    app.messages.push(format!("Connection error: {}", e));
                                } else {
                                    let mut app = app.lock().await;
                                    app.connected = true;
                                    app.current_field = InputField::Message;
                                    app.input_mode = InputMode::Editing;
                                    app.messages.push(String::from("Connected to server!"));
                                }
                            }
                        }
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Enter => {
                            if app_lock.connected && !app_lock.current_message.is_empty() {
                                let message = format!("<{}> {}", app_lock.nickname, app_lock.current_message);
                                app_lock.messages.push(message);
                                app_lock.current_message.clear();
                            }
                        }
                        KeyCode::Char(c) => {
                            match app_lock.current_field {
                                InputField::Nickname => app_lock.nickname.push(c),
                                InputField::Hostname => app_lock.hostname.push(c),
                                InputField::Channel => app_lock.channel.push(c),
                                InputField::Password => app_lock.password.push(c),
                                InputField::Message => app_lock.current_message.push(c),
                            }
                        }
                        KeyCode::Backspace => {
                            match app_lock.current_field {
                                InputField::Nickname => { app_lock.nickname.pop(); }
                                InputField::Hostname => { app_lock.hostname.pop(); }
                                InputField::Channel => { app_lock.channel.pop(); }
                                InputField::Password => { app_lock.password.pop(); }
                                InputField::Message => { app_lock.current_message.pop(); }
                            }
                        }
                        KeyCode::Esc => {
                            app_lock.input_mode = InputMode::Normal;
                        }
                        _ => {}
                    },
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    if !app.connected {
        login_screen::render(f, app);
    } else {
        chat_screen::render(f, app);
    }
}
