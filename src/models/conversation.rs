use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub text: String,
    pub is_user: bool,
} // TODO: add more fields (e.g. timestamp, user name, id etc.)

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Conversation {
    pub messages: Vec<Message>,
} // TODO: add more fields (e.g. id etc.)

impl Conversation {
    pub fn new() -> Self {
        Self {
            messages: vec![
                Message {
                    text: "Hello, I am a chat bot!".to_string(),
                    is_user: false,
                },
                Message {
                    text: "Hello, I am a human!".to_string(),
                    is_user: true,
                },
            ],
        }
    }
}

impl Display for Conversation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for message in &self.messages {
            writeln!(f, "{}", message.text)?;
        }
        Ok(())
    }
}
