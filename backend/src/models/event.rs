use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum EventType {
    Training,
    Game,
    Tournament,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ParticipantStatus {
    Pending,
    Confirmed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Participant {
    pub user_id: Uuid,
    pub status: ParticipantStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(rename = "type")]
    pub event_type: EventType,
    pub location_id: Uuid,
    pub datetime: String, // Using String for simplicity, will be ISO 8601
    pub level: Option<String>,
    pub price: Option<u32>,
    pub trainer_id: Option<Uuid>,
    pub confirmed: bool,
    pub participants: Vec<Participant>,
} 