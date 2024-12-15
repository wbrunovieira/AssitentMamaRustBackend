use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct MessageInput {
    pub content: String,
}

#[derive(Serialize, Debug)]
pub struct MessageResponse {
    pub reply: String,
    pub additional_data: Option<serde_json::Value>
}

#[derive(Deserialize, Debug)]
pub struct CommandLogInput {
    pub command: String,
}

#[derive(Serialize, Debug)]
pub struct CommandLogResponse {
    pub status: String,
}
