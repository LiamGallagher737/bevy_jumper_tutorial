---
title: Background
description: Add a background to our game
layout: ../../layouts/MainLayout.astro
---

## Use the Asset Server

In the `startup` system, add another parameter of type `Res` containg `AssetServer`, we can use this to load any files within the `assets` folder. `Res` stands for resource and is what you use for sharing data with lot's of diferent systems. This resource will let us access the apps asset server which we can use to load files from the [assets folder](../getting-started/asset-setup).

```rs
fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera Spawning Code
}
```

## Load the Image

We can load the images by using `asset_server.load()` and inputting the path of the fle relative to the assets folder, in our case the following will do.

```rs
let background_1 = asset_server.load("Background/bg_layer1.png");
```

The `let` keyword is use to initalize a variable. Here we are setting the `background_1` variable equal to the result of the `asset_server.load()` function. In our case the result is a `Handle<Image>`.

## Spawn the Background Entities

We can then use the following to spawn a entity with the background image set as its texture.

```rs
commands.spawn_bundle(SpriteBundle {
    texture: background_1,
    ..Default::default()
});
```

Do this 3 more times to spawn in each layer of the background.

```rs
let background_1 = asset_server.load("Background/bg_layer1.png");
commands.spawn_bundle(SpriteBundle {
    texture: background_1,
    ..Default::default()
});

let background_2 = asset_server.load("Background/bg_layer2.png");
commands.spawn_bundle(SpriteBundle {
    texture: background_2,
    ..Default::default()
});

let background_3 = asset_server.load("Background/bg_layer3.png");
commands.spawn_bundle(SpriteBundle {
    texture: background_3,
    ..Default::default()
});

let background_4 = asset_server.load("Background/bg_layer4.png");
commands.spawn_bundle(SpriteBundle {
    texture: background_4,
    ..Default::default()
});
```

But this is quite messy, so instead we can tidy it up by using a rust [closure](https://doc.rust-lang.org/rust-by-example/fn/closures.html). Add the following code.

```rs
let mut spawn_background = |path| {
    let handle = asset_server.load(path);
    commands.spawn_bundle(SpriteBundle {
        texture: handle,
        ..Default::default()
    });
};
```

This basicly stores some code inside of a variable, the closure variable must be `mut` in our case as the `commands` variable is edited within it.
Now we can use the following to spawn in our layers.

```rs
spawn_background("Background/bg_layer1.png");
spawn_background("Background/bg_layer2.png");
spawn_background("Background/bg_layer3.png");
spawn_background("Background/bg_layer4.png");
```

Run your app and you should have a nice looking layered background.

## Resizing Background

Currently if you makethe window too big the background dosn't scale with it, we can add a new system that resizes the background images everytime the window size changes. Start by creating a new function and adding a paramter of type `Res<Events<WindowResized>>`.

```rs
fn background_resizer(events: Res<Events<WindowResized>>) {
    
}
```

The `WindowResized` isn't included in the `bevy::prelude::*` soyou will have to add the following at the top of your file to use it.

```rs
use bevy::window::WindowResized;
```

We can then loop over the window resized events by adding the following.

```rs
let mut reader = events.get_reader();
for event in reader.iter(&events) {
    
}
```

But how can we access the entites to edit them? Easy, you use a `Query`, a query will give you all the entities that have a specified set of components. The scale field we want to change is in the `Transform` component so add the following parameter. You need to make sure you add the `mut` as we want to edit the values on ths component. And as the parameters arer getting kind of long I'm going to seperate them onto seperate lines.

```rs
fn background_resizer(
    events: Res<Events<WindowResized>>, 
    mut query: Query<&mut Transform>,
) {
    let mut reader = events.get_reader();
    for event in reader.iter(&events) {
        
    }
}
```

We can now loop over the query and update the sizes, add the following inside of the loop.

```rs
for mut sprite in query.iter_mut() {
    sprite.custom_size = Some(Vec2::new(event.width, event.height));
}
```

But now the background streches it ways is wasn't ment to so we can instead use the following.

```rs
let x_scale = event.width / 1920.0;
let y_scale = event.height / 1080.0;
let scale = x_scale.max(y_scale);
transform.scale = Vec3::new(scale, scale, 1.0);
```

This will find out how much it needs to scale to cover the screen on the x axis and the y axis and then uses the larger number to set the scale. However now it won't look like it's doing anything at all, this is because the camera entity also has a `Transform` component meaning it's also being scaled. To combat we can create our own custom component to distinguish between background entities and other entites. Start by adding the following code outside of any functions.

```rs
#[derive(Component)]
struct BackgroundTag;
```

This is how you can create your own components. Back where we create the background entites, update it to also add our new component.

```rs
commands.spawn_bundle(SpriteBundle { // Code from earlier
    texture: handle, // Code from earlier
    ..Default::default() // Code from earlier
}).insert(BackgroundTag);
//^^^^^^^^^^^^^^^^^^^^^^ Add this
```

Now edit the `background_resizer` `query` to include the new component.

```rs
mut query: Query<&mut Transform, With<BackgroundTag>>,
```

As our component dosen't have any data we want to access we use the `With<>` item
Now if you try running your app the background should cover the entire scrren no matter the size of the window.
