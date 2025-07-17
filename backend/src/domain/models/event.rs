use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use super::{EventType, EventLevel, ParticipantStatus, PaymentStatus};

/// Participant in an event
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Participant {
    pub user_id: ObjectId,
    pub status: ParticipantStatus,
    pub payment: PaymentStatus,
}

/// Domain model for Event
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(rename = "type")]
    pub event_type: EventType,
    pub location_id: ObjectId,
    pub datetime: DateTime<Utc>,
    pub level: Option<EventLevel>,
    pub price: Option<u32>,
    pub trainer_id: Option<ObjectId>,
    pub confirmed: bool,
    #[serde(default)]
    pub participants: Vec<Participant>,
    pub max_participants: Option<u32>,
}

impl Event {
    pub fn new(event_type: EventType, location_id: ObjectId, datetime: DateTime<Utc>) -> Self {
        Self {
            id: None,
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

    pub fn add_participant(&mut self, user_id: ObjectId) -> Result<(), &'static str> {
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