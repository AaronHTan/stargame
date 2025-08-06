use bevy::prelude::*;

use crate::systems::graphics::GraphicsPlugin;
use crate::systems::input::InputPlugin;

pub struct Renderer;

impl Plugin for Renderer {
    fn build(&self, app: &mut App) {
        app.add_plugins((GraphicsPlugin, InputPlugin));
    }
}