use std::fmt;

pub mod user;
pub mod location;
pub mod event;
pub mod event_timer;
pub mod subscription;

use serde::{Deserialize, Serialize};

/// User roles in the system
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Default, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    #[default]
    User,
}

impl AsRef<str> for UserRole {
    fn as_ref(&self) -> &str {
        match self {
            UserRole::Admin => "admin",
            UserRole::User => "user",
        }
    }
}

impl TryFrom<String> for UserRole {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "admin" => Ok(UserRole::Admin),
            "user" => Ok(UserRole::User),
            _ => Err(format!("Invalid user role: {}", value)),
        }
    }
}

/// Event types
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, sqlx::Type, Default)]
#[sqlx(type_name = "event_type", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum EventType {
    #[default]
    Training,
    Game,
    Tournament,
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryFrom<String> for EventType {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "training" => Ok(EventType::Training),
            "game" => Ok(EventType::Game),
            "tournament" => Ok(EventType::Tournament),
            _ => Err(format!("Invalid event type: {}", value)),
        }
    }
}

impl AsRef<str> for EventType {
    fn as_ref(&self) -> &str {
        match self {
            EventType::Training => "training",
            EventType::Game => "game",
            EventType::Tournament => "tournament",
        }
    }
}

/// Participant status in an event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, sqlx::Type, Default)]
#[sqlx(type_name = "participant_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ParticipantStatus {
    #[default]
    Pending,
    Confirmed,
}

impl TryFrom<String> for ParticipantStatus {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "pending" => Ok(ParticipantStatus::Pending),
            "confirmed" => Ok(ParticipantStatus::Confirmed),
            _ => Err(format!("Invalid participant status: {}", value)),
        }
    }
}

impl AsRef<str> for ParticipantStatus {
    fn as_ref(&self) -> &str {
        match self {
            ParticipantStatus::Pending => "pending",
            ParticipantStatus::Confirmed => "confirmed",
        }
    }
}

/// Payment status
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, sqlx::Type, Default)]
#[sqlx(type_name = "payment_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum PaymentStatus {
    #[default]
    Manual,
    Paid,
    Unpaid,
}

impl TryFrom<String> for PaymentStatus {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "manual" => Ok(PaymentStatus::Manual),
            "paid" => Ok(PaymentStatus::Paid),
            "unpaid" => Ok(PaymentStatus::Unpaid),
            _ => Err(format!("Invalid payment status: {}", value)),
        }
    }
}

impl AsRef<str> for PaymentStatus {
    fn as_ref(&self) -> &str {
        match self {
            PaymentStatus::Manual => "manual",
            PaymentStatus::Paid => "paid",
            PaymentStatus::Unpaid => "unpaid",
        }
    }
}

/// Event level
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, sqlx::Type, Default)]
#[sqlx(type_name = "event_level", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum EventLevel {
    #[default]
    Beginner,
    Intermediate,
    Advanced,
    Pro,
}

impl TryFrom<String> for EventLevel {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "beginner" => Ok(EventLevel::Beginner),
            "intermediate" => Ok(EventLevel::Intermediate),
            "advanced" => Ok(EventLevel::Advanced),
            "pro" => Ok(EventLevel::Pro),
            _ => Err(format!("Invalid event level: {}", value)),
        }
    }
}

impl AsRef<str> for EventLevel {
    fn as_ref(&self) -> &str {
        match self {
            EventLevel::Beginner => "beginner",
            EventLevel::Intermediate => "intermediate",
            EventLevel::Advanced => "advanced",
            EventLevel::Pro => "pro",
        }
    }
} 