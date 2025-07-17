use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use super::{EventType, EventLevel};
use crate::domain::models::{ParticipantStatus, PaymentStatus};
use sqlx::types::uuid::Uuid;

/// Participant in an event
#[derive(Debug, Serialize, Deserialize, Clone, Default, sqlx::FromRow)]
pub struct Participant {
    pub user_id: Uuid,
    #[sqlx(try_from = "String")]
    pub status: ParticipantStatus,
    #[sqlx(try_from = "String")]
    pub payment: PaymentStatus,
}

/// Domain model for Event
#[derive(Debug, Serialize, Deserialize, Clone, Default, sqlx::FromRow)]
pub struct Event {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    #[sqlx(try_from = "String")]
    pub event_type: EventType,
    pub location_id: Uuid,
    pub datetime: DateTime<Utc>,
    pub level: Option<String>,
    pub price: Option<i32>,
    pub trainer_id: Option<Uuid>,
    pub confirmed: bool,
    #[sqlx(json)]
    #[serde(default)]
    pub participants: Vec<Participant>,
    pub max_participants: Option<i32>,
}

impl Event {
    pub fn new(
        name: String,
        description: Option<String>,
        event_type: EventType,
        location_id: Uuid,
        datetime: DateTime<Utc>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            event_type,
            location_id,
            datetime,
            level: None,
            price: None,
            trainer_id: None,
            confirmed: false,
            participants: Vec::new(),
            max_participants: None,
        }
    }

    pub fn is_full(&self) -> bool {
        if let Some(max) = self.max_participants {
            self.participants.len() >= max as usize
        } else {
            false
        }
    }

    pub fn add_participant(&mut self, user_id: Uuid) -> Result<(), &'static str> {
        if self.is_full() {
            return Err("Event is full");
        }

        if self.participants.iter().any(|p| p.user_id == user_id) {
            return Err("User already registered");
        }

        self.participants.push(Participant {
            user_id,
            status: ParticipantStatus::Pending,
            payment: PaymentStatus::Unpaid,
        });

        Ok(())
    }
} 