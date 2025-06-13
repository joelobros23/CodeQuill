// src/models.rs

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    // Add other user-related fields like email, password hash, etc.
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Document {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub title: String,
    pub content: String,
    // Add metadata like creation date, last modified date, etc.
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: Uuid,
    pub sender_id: Uuid,
    pub document_id: Uuid,
    pub content: String,
    // Add timestamp, message type, etc.
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CollaborationRequest {
    pub id: Uuid,
    pub document_id: Uuid,
    pub sender_id: Uuid,
    pub receiver_id: Uuid,
    pub status: CollaborationRequestStatus, // e.g., Pending, Accepted, Rejected
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CollaborationRequestStatus {
    Pending,
    Accepted,
    Rejected,
}
