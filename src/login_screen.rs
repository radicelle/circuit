use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
    style::{Style, Color},
};
use crate::app::{App, InputField, InputMode};

pub fn render(f: &mut Frame, app: &App) {
    // Create a block for the entire form
    let form_block = Block::default()
        .borders(Borders::ALL)
        .title("IRC Client Login")
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Color::Cyan));

    // Create a layout for the header and form section
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)  // Add margin from the outer border
        .constraints([
            Constraint::Length(5),  // Increased header height
            Constraint::Min(20),    // Form section with minimum height
        ])
        .split(f.size());

    // Render the main block
    f.render_widget(form_block, f.size());

    // Render header with a border
    let header = Paragraph::new("Welcome to IRC Client\nPlease enter your connection details")
        .alignment(Alignment::Center)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue))
            .title("Welcome")
            .title_alignment(Alignment::Center))
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(header, main_layout[0]);

    // Create a block for the form section
    let form_section = Block::default()
        .borders(Borders::ALL)
        .title("Connection Details")
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Color::Blue));

    let form_area = main_layout[1];
    f.render_widget(form_section.clone(), form_area);

    // Create inner layout for form fields and button
    let inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)  // Add margin inside the form section
        .constraints([
            Constraint::Length(12), // Form fields
            Constraint::Length(3),  // Connect button
            Constraint::Min(0),     // Remaining space
        ])
        .split(form_section.inner(form_area));

    // Create form fields layout
    let form_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Nickname
            Constraint::Length(3), // Hostname
            Constraint::Length(3), // Channel
            Constraint::Length(3), // Password
        ])
        .split(inner_layout[0]);

    // Render form fields with highlighted active field
    let fields = [
        (InputField::Nickname, &app.nickname, "Nickname"),
        (InputField::Hostname, &app.hostname, "Hostname"),
        (InputField::Channel, &app.channel, "Channel"),
        (InputField::Password, &app.password, "Password"),
    ];

    for (i, (field, value, title)) in fields.iter().enumerate() {
        let is_active = app.current_field == *field;
        let field_style = if is_active {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        let content = if *field == InputField::Password {
            "*".repeat(value.len())
        } else {
            value.to_string()
        };

        let paragraph = Paragraph::new(content)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(field_style)
                .title(title.to_string()));
        
        f.render_widget(paragraph, form_chunks[i]);
    }

    // Render connect button
    let button_style = if app.input_mode == InputMode::Normal {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let connect_button = Paragraph::new("[ Connect ]")
        .alignment(Alignment::Center)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(button_style));
    f.render_widget(connect_button, inner_layout[1]);

    // Show cursor in edit mode
    if app.input_mode == InputMode::Editing {
        match app.current_field {
            InputField::Nickname => {
                f.set_cursor(
                    form_chunks[0].x + app.nickname.len() as u16 + 1,
                    form_chunks[0].y + 1,
                )
            }
            InputField::Hostname => {
                f.set_cursor(
                    form_chunks[1].x + app.hostname.len() as u16 + 1,
                    form_chunks[1].y + 1,
                )
            }
            InputField::Channel => {
                f.set_cursor(
                    form_chunks[2].x + app.channel.len() as u16 + 1,
                    form_chunks[2].y + 1,
                )
            }
            InputField::Password => {
                f.set_cursor(
                    form_chunks[3].x + app.password.len() as u16 + 1,
                    form_chunks[3].y + 1,
                )
            }
            _ => {}
        }
    }
} 