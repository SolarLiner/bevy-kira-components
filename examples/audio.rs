use bevy::prelude::*;

use bevy_kira_components::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AudioPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(AudioFileBundle {
        source: asset_server.load("Windless Slopes.ogg"),
        ..default()
    });
}
