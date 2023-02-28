pub use crate::prelude::*;

#[derive(Default, Copy, Clone, PartialEq, Debug)]
pub enum PlayerAnimationVariant {
    #[default]
    Idle,
    WalkRight,
    WalkLeft,
    WalkDown,
    WalkUp,
}

impl AnimationLoop for PlayerAnimationVariant {
    fn page(&self) -> (usize, usize) {
        match self {
            PlayerAnimationVariant::Idle => (0, 6),
            PlayerAnimationVariant::WalkRight | PlayerAnimationVariant::WalkLeft => (24, 6),
            PlayerAnimationVariant::WalkDown => (18, 6),
            PlayerAnimationVariant::WalkUp => (30, 6),
        }
    }
}

#[derive(Default, Clone, Component, AnimationTransitionMacro)]
pub struct PlayerAnimationState {
    #[variant]
    pub variant: PlayerAnimationVariant,
    pub idx: usize,
}
