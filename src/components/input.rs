use bevy::prelude::*;

#[derive(Debug, Component, Deref, DerefMut)]
pub struct CameraSensitivity(pub Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(
            Vec2::new(0.003, 0.002),
        )
    }
}