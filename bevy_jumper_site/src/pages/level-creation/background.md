---
title: Background
description: Add a background to our game
layout: ../../layouts/MainLayout.astro
---

## Create a Startup System

The first thing we need to do before we can spawn in our camera is to create a startup system. This system will run only once when the app first launches. Start by adding a neww function to your `main.rs`.

```rs
fn startup() {
    
}
```

Then add the following to the app creation.

```rs
.add_startup_system(startup)
```

Back in the startup system wewant to add a parameter to the function, this will be `Commands` whch will allow us to spawwn in new entities and we will make sure to add `mut` so we can edit it's value.

```rs
fn startup(mut commands: Commands) {

}
```

Now we can spawn in our cammera by adding the following in the `startup` system.

```rs
commands.spawn_bundle(Camera2dBundle::default());
```

Now when you run your app with `cargo run` you should notice that the background color is grey instead of black, this means the camera has be successfully spawned in.
