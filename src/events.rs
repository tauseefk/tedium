use crate::prelude::*;

// Add or remove a wall block
#[derive(Event)]
pub struct ToggleWallBlockEvent {
    pub translation: Vec3,
}

// Event to cycle through Points of Interest
#[derive(Event)]
pub struct CyclePOIEvent;

pub enum PlayerMoveDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Event)]
pub struct PlayerMoveEvent {
    pub direction: PlayerMoveDirection,
}
