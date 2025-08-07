use bevy::{
    color::palettes::tailwind, prelude::*, render::view::RenderLayers,
    pbr::NotShadowCaster,
};

use crate::components::graphics::{Player, WorldModelCamera, VIEW_MODEL_RENDER_LAYER, DEFAULT_RENDER_LAYER};
use crate::components::input::CameraSensitivity;

fn spawn_view_model(
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
            Transform::from_xyz(0.0, 1.0, 0.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
            Visibility::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                WorldModelCamera,
                Camera3d::default(),
                Projection::from(PerspectiveProjection {
                    fov: 60.0_f32.to_radians(),
                    aspect_ratio: 1.0,
                    near: 0.1,
                    far: 1000.0,
                }),
            ));

            parent.spawn((
                Camera3d::default(),
                Camera {
                    order: 1,
                    clear_color: ClearColorConfig::None,
                    ..default()
                },
                Projection::from(PerspectiveProjection {
                    fov: 60.0_f32.to_radians(),
                    aspect_ratio: 1.0,
                    near: 0.01,
                    far: 10.0,
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

fn spawn_world_model(
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
        Mesh3d(cube.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(1.5, 0.25, -3.0),
        RenderLayers::layer(DEFAULT_RENDER_LAYER),
    ));
    
    commands.spawn((
        Mesh3d(cube.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(1.0, 0.25, -1.0),
        RenderLayers::layer(DEFAULT_RENDER_LAYER),
    ));
    
    commands.spawn((
        Mesh3d(cube.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(0.0, 0.25, 3.0),
        RenderLayers::layer(DEFAULT_RENDER_LAYER),
    ));
       
    commands.spawn((
        Mesh3d(cube),
        MeshMaterial3d(material),
        Transform::from_xyz(0.75, 1.75, 0.0),
        RenderLayers::layer(DEFAULT_RENDER_LAYER),
    ));
}

fn spawn_lights(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let light_position = Vec3::new(4.0, 8.0, 4.0);
    
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(0.98, 0.95, 0.82),
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_translation(light_position)
            .looking_at(Vec3::ZERO, Vec3::Y),
        RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
    ));

    let sphere = meshes.add(Sphere::new(0.3));
    let emissive_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.95, 0.7),
        emissive: LinearRgba::rgb(10.0, 9.5, 7.0),
        ..default()
    });

    commands.spawn((
        Mesh3d(sphere),
        MeshMaterial3d(emissive_material),
        Transform::from_translation(light_position),
        RenderLayers::layer(DEFAULT_RENDER_LAYER),
        NotShadowCaster,
    ));

    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.5, 0.5, 0.5),
        brightness: 200.0,
        affects_lightmapped_meshes: true,
    });
}

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (spawn_view_model, spawn_world_model, spawn_lights)
        );
    }
}