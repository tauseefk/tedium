use crate::prelude::*;

// Add or remove a wall block
pub struct ToggleWallBlockEvent {
    pub translation: Vec3,
}

// Event to cycle through Points of Interest
pub struct CyclePOIEvent;
