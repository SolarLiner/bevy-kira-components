use bevy::color::palettes::css::{GRAY, GREEN};
use bevy::math::vec3;
use bevy::prelude::*;

use bevy_kira_components::kira::spatial::emitter::EmitterDistances;
use bevy_kira_components::prelude::*;
use bevy_kira_components::AudioPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AudioPlugin))
        .add_systems(Startup, (init_camera, init_objects))
        .add_systems(Update, rotate_objects)
        // .add_systems(
        //     PostUpdate,
        //     on_audio_file_removed.after(AudioPlaybackSet::Cleanup),
        // )
        .run();
}

fn init_camera(mut commands: Commands) {
    let transform = Transform::from_xyz(0., 2., 0.).looking_at(vec3(0., 1., -6.), Vec3::Y);
    commands.spawn((
        AudioListener,
        Camera3dBundle {
            transform,
            ..default()
        },
    ));
}

#[derive(Component)]
struct Rotate(Quat);

fn init_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let source = asset_server.load("drums.ogg");
    // Audio emitter
    commands
        .spawn((
            Rotate(Quat::from_rotation_y(1.0)),
            InheritedVisibility::VISIBLE,
            TransformBundle {
                local: Transform::from_xyz(0., 1., -6.0),
                ..default()
            },
        ))
        .with_children(|children| {
            children.spawn((
                SpatialEmitter {
                    distances: EmitterDistances {
                        min_distance: 1.0,
                        max_distance: 20.0,
                    },
                    ..default()
                },
                AudioFileBundle {
                    source,
                    settings: AudioFileSettings {
                        loop_region: None,
                        ..default()
                    },
                    ..default()
                },
                AudioFileEndBehavior::RemoveComponents,
                PbrBundle {
                    mesh: meshes.add(Sphere::new(0.1).mesh()),
                    material: materials.add(StandardMaterial {
                        base_color: Color::WHITE,
                        emissive: GREEN.into(),
                        ..default()
                    }),
                    transform: Transform::from_xyz(0., 0., 2.5),
                    ..default()
                },
            ));
        });

    // Plane
    commands.spawn(PbrBundle {
        transform: Transform::from_scale(Vec3::splat(100.0)),
        mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::ONE).mesh()),
        material: materials.add(StandardMaterial {
            base_color: GRAY.into(),
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

fn on_audio_file_removed(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut removed: RemovedComponents<Handle<AudioFile>>,
) {
    for entity in removed.read() {
        info!("Audio file removed: {:?}", entity);

        let source = asset_server.load("Windless Slopes.ogg");

        commands.entity(entity).insert((
            AudioFileBundle {
                source,
                settings: AudioFileSettings {
                    loop_region: None,
                    ..default()
                },
                ..default()
            },
            AudioFileEndBehavior::RemoveComponents,
        ));
    }
}

fn rotate_objects(time: Res<Time>, mut q: Query<(&mut Transform, &Rotate)>) {
    let dt = time.delta_seconds();
    if dt < 1e-6 {
        return;
    }
    for (mut transform, rotate) in &mut q {
        let quat = Quat::IDENTITY.slerp(rotate.0, dt);
        transform.rotate(quat);
    }
}
