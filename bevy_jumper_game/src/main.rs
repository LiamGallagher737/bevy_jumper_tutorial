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
        .run();
}