use serde::Deserialize;
use serde::Serialize;

// The conversation struct
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Conversation {
    pub messages: Vec<Message>
}

// This implementation will help us create new chats with ease
impl Conversation {
    pub fn new() -> Conversation {
//   initialise an empty conversation struct with an empty messages vector
        Conversation {
            messages: Vec::new()
        }
    }
}

// The basic message would contain a bool to represent message ownership
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message { 
    pub user:bool,
    pub text: String,
}