use bevy::prelude::*;

#[derive(Component)]
struct MyCameraMarker;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(), 
        Transform::from_xyz(10.0, 12.0,16.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        MyCameraMarker,
    ));
}

pub fn setup() {
    
}