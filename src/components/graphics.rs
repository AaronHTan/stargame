use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Player;

#[derive(Debug, Component)]
pub struct WorldModelCamera;

pub const DEFAULT_RENDER_LAYER: usize = 0;
pub const VIEW_MODEL_RENDER_LAYER: usize = 1;