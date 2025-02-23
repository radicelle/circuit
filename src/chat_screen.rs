use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use crate::app::{App, InputField, InputMode};

pub fn render(f: &mut Frame, app: &App) {
    let messages_and_input = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(80),
            Constraint::Percentage(20),
        ])
        .split(f.size());

    let message_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(messages_and_input[0]);

    // Display users list
    let users: Vec<ListItem> = app
        .users
        .iter()
        .map(|u| ListItem::new(u.as_str()))
        .collect();
    let users_list = List::new(users)
        .block(Block::default().borders(Borders::ALL).title("Users"));
    f.render_widget(users_list, messages_and_input[1]);

    // Chat interface
    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .map(|m| ListItem::new(m.as_str()))
        .collect();
    let messages = List::new(messages)
        .block(Block::default().borders(Borders::ALL).title("Messages"));
    f.render_widget(messages, message_area[0]);

    let input = Paragraph::new(app.current_message.as_str())
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, message_area[1]);

    // Cursor positioning for chat
    if app.input_mode == InputMode::Editing && matches!(app.current_field, InputField::Message) {
        f.set_cursor(
            message_area[1].x + app.current_message.len() as u16 + 1,
            message_area[1].y + 1
        );
    }
} 