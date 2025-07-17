use serde::{Serialize, Deserialize};
use super::{EventType, EventLevel};
use sqlx::types::uuid::Uuid;
use sqlx::types::chrono::NaiveTime;

/// Repeat pattern for event timers
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "repeat_pattern", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum RepeatPattern {
    Weekly,
    Daily,
    Monthly,
}

/// Domain model for EventTimer (recurring event template)
#[derive(Debug, Serialize, Deserialize, Clone, Default, sqlx::FromRow)]
pub struct EventTimer {
    pub id: Uuid,
    pub name: String,
    pub location_id: Uuid,
    #[sqlx(try_from = "String")]
    pub event_type: EventType,
    pub level: Option<String>,
    pub day_of_week: i32, // 1-7 for Monday-Sunday
    pub time: NaiveTime,
    pub active: bool,
    pub price: Option<i32>,
    pub max_participants: Option<i32>,
}

impl EventTimer {
    pub fn new(
        name: String,
        location_id: Uuid,
        event_type: EventType,
        day_of_week: i32,
        time: NaiveTime,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            location_id,
            event_type,
            level: None,
            day_of_week,
            time,
            active: true,
            price: None,
            max_participants: None,
        }
    }
} 