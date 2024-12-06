use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct MessageInput {
    pub content: String,
}

#[derive(Serialize, Debug)]
pub struct MessageResponse {
    pub reply: String,
}

#[derive(Deserialize, Debug)]
pub struct CommandLogInput {
    pub command: String,
}

#[derive(Serialize, Debug)]
pub struct CommandLogResponse {
    pub status: String,
}
