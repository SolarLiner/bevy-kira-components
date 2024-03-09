use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;

use bevy_kira_components::kira::sound::Region;
use bevy_kira_components::spatial::{AudioListener, SpatialEmitter};
use bevy_kira_components::{Audio, AudioLoaderSettings, AudioPlugin};

use crate::camera::{CameraPlugin, FpsCam};

mod camera;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            AudioPlugin,
            LogDiagnosticsPlugin::default(),
            CameraPlugin,
        ))
        .add_systems(Startup, (init_camera, init_objects))
        .run();
}

fn init_camera(mut commands: Commands) {
    let transform = Transform::from_xyz(2., 2., 5.);
    commands.spawn((
        AudioListener,
        FpsCam::default(),
        Camera3dBundle {
            transform,
            ..default()
        },
    ));
}

fn init_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Audio emitter
    commands.spawn((
        SpatialEmitter,
        Audio::new(
            asset_server.load_with_settings("drums.ogg", |s: &mut AudioLoaderSettings| {
                s.looping = Some(Region::from(3.6..6.0));
            }),
        ),
        PbrBundle {
            mesh: meshes.add(Sphere::new(0.1).mesh()),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                emissive: Color::GREEN,
                ..default()
            }),
            transform: Transform::from_xyz(0., 1.0, -6.0),
            ..default()
        },
    ));

    // Plane
    commands.spawn(PbrBundle {
        transform: Transform::from_scale(Vec3::splat(100.0)),
        mesh: meshes.add(Plane3d::new(Vec3::Y).mesh()),
        material: materials.add(StandardMaterial {
            base_color: Color::GRAY,
            ..default()
        }),
        ..default()
    });

    // Sun
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::default().looking_at(Vec3::NEG_Y, Vec3::Z),
        ..default()
    });
}
