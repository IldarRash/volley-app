use std::fmt;

pub mod user;
pub mod location;
pub mod event;
pub mod event_timer;
pub mod subscription;

use serde::{Deserialize, Serialize};

/// User roles in the system
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    #[default]
    User,
}

/// Event types
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EventType {
    Training,
    Game,
    Tournament,
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Participant status in an event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ParticipantStatus {
    Pending,
    Confirmed,
}

/// Payment status
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PaymentStatus {
    Manual,
    Paid,
    Unpaid,
}

/// Event level
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EventLevel {
    Beginner,
    Intermediate,
    Advanced,
    Pro,
} 