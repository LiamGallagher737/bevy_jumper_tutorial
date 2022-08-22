use bevy::{prelude::*, window::WindowResized};

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
        .add_system(background_resizer)
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

    let mut spawn_background = |path| {
        let handle = asset_server.load(path);
        commands.spawn_bundle(SpriteBundle {
            texture: handle,
            ..Default::default()
        }).insert(BackgroundTag);
    };

    spawn_background("Background/bg_layer1.png");
    spawn_background("Background/bg_layer2.png");
    spawn_background("Background/bg_layer3.png");
    spawn_background("Background/bg_layer4.png");
}

fn background_resizer(
    events: Res<Events<WindowResized>>, 
    mut query: Query<&mut Transform, With<BackgroundTag>>,
) {
    let mut reader = events.get_reader();
    for event in reader.iter(&events) {
        for mut transform in query.iter_mut() {
            let x_scale = event.width / 1920.0;
            let y_scale = event.height / 1080.0;
            let scale = x_scale.max(y_scale);
            transform.scale = Vec3::new(scale, scale, 1.0);
        }
    }
}

#[derive(Component)]
struct BackgroundTag;
