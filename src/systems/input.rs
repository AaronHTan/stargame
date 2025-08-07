use std::{f32::consts::FRAC_PI_2};

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

    // W moves forward (in the direction camera is looking)
    if keyboard_input.pressed(KeyCode::KeyW) {
        movement.z -= 1.0;
    }
    // S moves backward (opposite to where camera is looking)
    if keyboard_input.pressed(KeyCode::KeyS) {
        movement.z += 1.0;
    }
    // A and D for strafing left/right
    if keyboard_input.pressed(KeyCode::KeyA) {
        movement.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        movement.x += 1.0;
    }
    // Space moves up
    if keyboard_input.pressed(KeyCode::Space) {
        movement.y += 1.0;
    }
    // Shift moves down
    if keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.pressed(KeyCode::ShiftRight) {
        movement.y -= 1.0;
    }

    if movement.length_squared() > 0.0 {
        movement = movement.normalize();
        let forward = player.forward();
        let right = player.right();
        let up = player.up();
        
        // Camera-relative movement: transforms movement from local to world space
        let translation_delta = (forward * -movement.z + right * movement.x + up * movement.y) * move_speed;
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
            if let Projection::Perspective(perspective) = projection.as_mut() {
                let zoom_speed = 0.05;
                let zoom_factor = 1.0 - scroll_delta * zoom_speed;
                
                perspective.fov *= zoom_factor;
                perspective.fov = perspective.fov.clamp(15.0_f32.to_radians(), 120.0_f32.to_radians());
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