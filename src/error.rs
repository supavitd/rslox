#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub line: u32,
}

impl Error {
    pub fn new(message: &str, line: u32) -> Self {
        Self {
            message: message.to_string(),
            line,
        }
    }
}
