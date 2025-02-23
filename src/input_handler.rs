use crate::app::{App, InputField, InputMode};
use crossterm::event::{KeyCode, KeyModifiers};


pub async fn handle_input(key: KeyCode, modifiers: KeyModifiers, app: &mut App) -> Option<InputAction> {
    match app.input_mode {
        InputMode::Normal => handle_normal_mode(key, modifiers, app),
        InputMode::Editing => handle_editing_mode(key, app),
    }
}

fn handle_normal_mode(key: KeyCode, modifiers: KeyModifiers, app: &mut App) -> Option<InputAction> {
    match key {
        KeyCode::Char('c') if modifiers == KeyModifiers::CONTROL => {
            Some(InputAction::Quit)
        }
        KeyCode::Char('e') => {
            app.input_mode = InputMode::Editing;
            None
        }
        KeyCode::Char('q') => Some(InputAction::Quit),
        KeyCode::Tab => {
            if !app.connected {
                app.current_field = match app.current_field {
                    InputField::Nickname => InputField::Hostname,
                    InputField::Hostname => InputField::Channel,
                    InputField::Channel => InputField::Password,
                    InputField::Password => InputField::Nickname,
                    InputField::Message => InputField::Message,
                };
            }
            None
        }
        KeyCode::Enter => {
            if should_try_connect(app) {
                Some(InputAction::TryConnect)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn handle_editing_mode(key: KeyCode, app: &mut App) -> Option<InputAction> {
    match key {
        KeyCode::Enter => {
            if app.connected && !app.current_message.is_empty() {
                let message = format!("<{}> {}", app.nickname, app.current_message);
                app.messages.push(message);
                app.current_message.clear();
            }
            None
        }
        KeyCode::Char(c) => {
            match app.current_field {
                InputField::Nickname => app.nickname.push(c),
                InputField::Hostname => app.hostname.push(c),
                InputField::Channel => app.channel.push(c),
                InputField::Password => app.password.push(c),
                InputField::Message => app.current_message.push(c),
            }
            None
        }
        KeyCode::Backspace => {
            match app.current_field {
                InputField::Nickname => { app.nickname.pop(); }
                InputField::Hostname => { app.hostname.pop(); }
                InputField::Channel => { app.channel.pop(); }
                InputField::Password => { app.password.pop(); }
                InputField::Message => { app.current_message.pop(); }
            }
            None
        }
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
            None
        }
        _ => None,
    }
}

fn should_try_connect(app: &App) -> bool {
    !app.connected
        && app.input_mode == InputMode::Editing
        && matches!(app.current_field, InputField::Password)
        && !app.nickname.is_empty()
        && !app.hostname.is_empty()
        && !app.channel.is_empty()
}

pub enum InputAction {
    Quit,
    TryConnect,
} 