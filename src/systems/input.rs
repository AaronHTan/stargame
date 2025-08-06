use std::f32::consts::FRAC_PI_2;

use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    prelude::*,
};

use crate::components::graphics::{Player, WorldModelCamera};
use crate::components::input::CameraSensitivity;

pub fn move_player_mouse(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    player: Single<(&mut Transform, &CameraSensitivity), With<Player>>,
) {
    let (mut transform, camera_sensitivity) = player.into_inner();

    let delta = accumulated_mouse_motion.delta;

    if delta != Vec2::ZERO {
        let delta_yaw = -delta.x * camera_sensitivity.x;
        let delta_pitch = -delta.y * camera_sensitivity.y;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}

pub fn move_player_keyboard(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Single<&mut Transform, With<Player>>,
) {
    let mut movement = Vec3::ZERO;
    let move_speed = 5.0 * time.delta_secs();

    if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
        movement.z -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
        movement.z += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
        movement.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
        movement.x += 1.0;
    }

    if movement.length_squared() > 0.0 {
        movement = movement.normalize();
        let forward = player.forward();
        let right = player.right();
        
        let translation_delta = (forward * -movement.z + right * movement.x) * move_speed;
        player.translation += translation_delta;
    }
}

pub fn zoom_camera(
    accumulated_mouse_scroll: Res<AccumulatedMouseScroll>,
    mut projection_query: Query<&mut Projection, With<WorldModelCamera>>,
) {
    if let Ok(mut projection) = projection_query.single_mut() {
        let scroll_delta = accumulated_mouse_scroll.delta.y;
        
        if scroll_delta != 0.0 {
            if let Projection::Orthographic(ortho) = projection.as_mut() {
                let zoom_speed = 0.1;
                let zoom_factor = 1.0 - scroll_delta * zoom_speed;
                
                ortho.scale *= zoom_factor;
                ortho.scale = ortho.scale.clamp(0.01, 0.5);
            }
        }
    }
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (move_player_mouse, move_player_keyboard, zoom_camera)
        );
    }
}