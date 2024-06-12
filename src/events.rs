use crate::prelude::*;

// Add or remove a wall block
pub struct ToggleWallBlockEvent {
    pub translation: Vec3,
}

pub struct UpdateDebugValuesEvent {
    pub translation: Vec3,
}

// Event to cycle through Points of Interest
pub struct CyclePOIEvent;

pub enum PlayerMoveDirection {
    Up,
    Down,
    Left,
    Right,
}

pub struct PlayerMoveEvent {
    pub direction: PlayerMoveDirection,
}
