use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use super::EventType;

/// Subscription/Pass model for users
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Subscription {
    pub trainer_id: Option<ObjectId>,
    pub weekday: u8, // 1-7 for Monday-Sunday
    pub time: String, // "18:00" format
    pub valid_until: DateTime<Utc>,
    pub event_type: EventType,
}

impl Subscription {
    pub fn is_valid(&self) -> bool {
        Utc::now() < self.valid_until
    }

    pub fn matches_event(&self, trainer_id: Option<ObjectId>, weekday: u8, time: &str, event_type: &EventType) -> bool {
        self.is_valid() 
            && self.trainer_id == trainer_id
            && self.weekday == weekday
            && self.time == time
            && &self.event_type == event_type
    }
} 