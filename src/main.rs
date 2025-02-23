mod app;
mod login_screen;
mod chat_screen;
mod irc_client;
mod input_handler;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
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
use input_handler::{handle_input, InputAction};

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
                let mut app_lock = app.lock().await;
                
                if let Some(action) = handle_input(key.code, key.modifiers, &mut app_lock).await {
                    match action {
                        InputAction::Quit => return Ok(()),
                        InputAction::TryConnect => {
                            drop(app_lock);
                            let app_clone = Arc::clone(&app);
                            if let Err(e) = irc_client::connect_to_server(app_clone.clone()).await {
                                let mut app = app_clone.lock().await;
                                app.messages.push(format!("Connection error: {}", e));
                            } else {
                                let mut app = app_clone.lock().await;
                                app.connected = true;
                                app.current_field = InputField::Message;
                                app.input_mode = InputMode::Editing;
                                app.messages.push(String::from("Connected to server!"));
                            }
                        }
                    }
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
