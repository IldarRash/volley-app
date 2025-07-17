// backend/src/models/mod.rs

pub mod user;

use serde::{Deserialize, Serialize};

/// Represents the possible positions a player can have.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PlayerPosition {
    Setter,
    Libero,
    MiddleBlocker,
    OutsideHitter,
    OppositeHitter,
}

/// Represents the properties of a player.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PlayerProps {
    Attacker,
    Defender,
    AllRounder,
}

/// Represents the gender of a user.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Gender {
    Male,
    Female,
    Other,
} 