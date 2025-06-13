// src/models.rs

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    // Add other user-related fields here (e.g., email, password hash)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Document {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub owner_id: Uuid, // Foreign key referencing User.id
    // Add other document-related fields here (e.g., creation_date, last_modified_date)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: Uuid,
    pub sender_id: Uuid, // Foreign key referencing User.id
    pub content: String,
    pub timestamp: String, // Consider using a DateTime type for better timestamp handling
    // Add other message-related fields here (e.g., recipient_id, conversation_id)
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
    pub fn new(sender_id: Uuid, content: String, timestamp: String) -> Self {
        Message {
            id: Uuid::new_v4(),
            sender_id,
            content,
            timestamp,
        }
    }
}