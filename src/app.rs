#[derive(PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

#[derive(PartialEq)]
pub enum InputField {
    Nickname,
    Hostname,
    Channel,
    Password,
    Message,
}

pub struct App {
    pub nickname: String,
    pub hostname: String,
    pub channel: String,
    pub password: String,
    pub current_field: InputField,
    pub input_mode: InputMode,
    pub messages: Vec<String>,
    pub current_message: String,
    pub connected: bool,
    pub users: Vec<String>,
}

impl Default for App {
    fn default() -> App {
        App {
            nickname: String::new(),
            hostname: String::new(),
            channel: String::new(),
            password: String::new(),
            current_field: InputField::Nickname,
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            current_message: String::new(),
            connected: false,
            users: Vec::new(),
        }
    }
} 