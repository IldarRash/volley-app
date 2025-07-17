use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::models::event::EventType;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventTimer {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(rename = "type")]
    pub event_type: EventType,
    pub level: Option<String>,
    pub repeat: String, // e.g., "weekly"
    pub weekday: Option<u32>, // 1-7 for Monday-Sunday
    pub hour: Option<u32>,
    pub location_id: Uuid,
    pub price: Option<u32>,
    pub max_participants: Option<u32>,
} 