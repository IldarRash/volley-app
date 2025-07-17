use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use super::{EventType, EventLevel};

/// Repeat pattern for event timers
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum RepeatPattern {
    Weekly,
    Daily,
    Monthly,
}

/// Domain model for EventTimer (recurring event template)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventTimer {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(rename = "type")]
    pub event_type: EventType,
    pub level: Option<EventLevel>,
    pub repeat: RepeatPattern,
    pub weekday: Option<u8>, // 1-7 for Monday-Sunday
    pub hour: u8, // 0-23
    pub minute: u8, // 0-59
    pub location_id: ObjectId,
    pub trainer_id: Option<ObjectId>,
    pub price: Option<u32>,
    pub max_participants: Option<u32>,
    pub description: Option<String>,
    pub active: bool,
}

impl EventTimer {
    pub fn new(
        event_type: EventType,
        repeat: RepeatPattern,
        hour: u8,
        minute: u8,
        location_id: ObjectId,
    ) -> Self {
        Self {
            id: None,
            event_type,
            level: None,
            repeat,
            weekday: None,
            hour,
            minute,
            location_id,
            trainer_id: None,
            price: None,
            max_participants: None,
            description: None,
            active: true,
        }
    }
} 