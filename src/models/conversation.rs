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
            messages: vec![],
        }
    }
}
