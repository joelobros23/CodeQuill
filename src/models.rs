// src/models.rs

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    // Add other user-related fields as needed (e.g., email, password hash)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Document {
    pub id: Uuid,
    pub title: String,
    pub content: String, // Store code or text content here
    pub owner_id: Uuid,   // User ID of the document owner
    // Add other document-related fields as needed (e.g., creation date, last modified date)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: Uuid,
    pub sender_id: Uuid,
    pub recipient_id: Uuid,
    pub content: String,
    // Add other message-related fields as needed (e.g., timestamp)
}

impl User {
    pub fn new(username: String) -> Self {
        User {
            id: Uuid::new_v4(),
            username,
        }
    }
}

impl Document {
    pub fn new(title: String, content: String, owner_id: Uuid) -> Self {
        Document {
            id: Uuid::new_v4(),
            title,
            content,
            owner_id,
        }
    }
}

impl Message {
    pub fn new(sender_id: Uuid, recipient_id: Uuid, content: String) -> Self {
        Message {
            id: Uuid::new_v4(),
            sender_id,
            recipient_id,
            content,
        }
    }
}
