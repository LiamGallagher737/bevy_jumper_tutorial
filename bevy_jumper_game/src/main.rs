use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "My Amazing Jumper Game".to_string(),
            width: 1280.0,
            height: 720.0,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup)
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
