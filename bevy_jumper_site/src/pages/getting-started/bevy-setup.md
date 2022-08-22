---
title: Bevy Setup
description: Setting up Bevy so we can use it to create our game
layout: ../../layouts/MainLayout.astro
---

## Adding Bevy to your Rust Project

After creating a Rust project using, open your `Cargo.toml` and add the following to your dependencies.

```toml
[dependencies]
bevy = "0.8"
```

Adding the following to your `Cargo.toml` will dramaticly increase your games performance while developing as it will compile you dependices with a high level of optimizations, this will take much longer to compile the first time but after that it won't take any extra time.

```toml
[profile.dev.package."*"]
opt-level = 3
```

## Creating a Bevy App

Now that we have Bevy added to our project we can crate an app with it. Add the following to your `main.rs`.

```rs
// This imports the most common Bevy utilities
use bevy::prelude::*;

fn main() {
    // Creates a new Bevy app
    App::new()
        // Adds required plugins most most use cases
        .add_plugins(DefaultPlugins)
        // Runs the app
        .run();
}
```

Now run the project with `cargo run`, as this will be the first time running this project compile with probally take quite a while but once it's done a window should open up.

## Customizing the Window

You can customize the look of your window by adding the `WindowDescriptor` resource to your app, make sure you add this before addng the `DefaultPlugins`. You can view all the available options [here](https://docs.rs/bevy/latest/bevy/window/struct.WindowDescriptor.html).

```rs
use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "My Amazing Platformer".to_string(),
            width: 1280.0,
            height: 720.0,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}
```
