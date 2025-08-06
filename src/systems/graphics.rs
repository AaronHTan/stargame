use bevy::{
    color::palettes::tailwind, prelude::*, render::view::RenderLayers,
};

use crate::components::graphics::{Player, WorldModelCamera, VIEW_MODEL_RENDER_LAYER, DEFAULT_RENDER_LAYER};
use crate::components::input::CameraSensitivity;

pub fn spawn_view_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let arm = meshes.add(Cuboid::new(0.1, 0.1, 0.5));
    let arm_material = materials.add(Color::from(tailwind::TEAL_200));

    commands
        .spawn((
            Player,
            CameraSensitivity::default(),
            Transform::from_xyz(10.0, 10.0, 10.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
            Visibility::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                WorldModelCamera,
                Camera3d::default(),
                Projection::from(OrthographicProjection {
                    scale: 0.05,
                    ..OrthographicProjection::default_3d()
                }),
            ));

            parent.spawn((
                Camera3d::default(),
                Camera {
                    order: 1,
                    ..default()
                },
                Projection::from(OrthographicProjection {
                    scale: 0.002,
                    ..OrthographicProjection::default_3d()
                }),
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
            ));

            parent.spawn((
                Mesh3d(arm),
                MeshMaterial3d(arm_material),
                Transform::from_xyz(0.0, -0.1, -0.55),
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
            ));
        });
}

pub fn spawn_world_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.0)));
    let cube = meshes.add(Cuboid::new(2.0, 0.5, 1.0));
    let material = materials.add(Color::WHITE);

    commands.spawn((
        Mesh3d(floor), 
        MeshMaterial3d(material.clone()),
        RenderLayers::layer(DEFAULT_RENDER_LAYER),
    ));

    commands.spawn((
        Mesh3d(cube.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(0.0, 0.25, -3.0),
        RenderLayers::layer(DEFAULT_RENDER_LAYER),
    ));

    commands.spawn((
        Mesh3d(cube),
        MeshMaterial3d(material),
        Transform::from_xyz(0.75, 1.75, 0.0),
        RenderLayers::layer(DEFAULT_RENDER_LAYER),
    ));
}

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (spawn_view_model, spawn_world_model)
        );
    }
}