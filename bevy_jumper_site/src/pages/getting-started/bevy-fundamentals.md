---
title: Bevy Fundamentals
description: The fundamentals of the Bevy game engine
layout: ../../layouts/MainLayout.astro
---

## Entities

### Overview

Bevy uses entities to define objects in your game. An entity is a thing that can hold many [components](#overview-1).

### Example Entity

Take a 2d player character for example, its entity could look something like the following. This is a simplified version.

```
📦Player Entity
 ┣ 📜Transform
 ┣ 📜Sprite
 ┣ 📜Movement
 ┗ 📜Player Tag
```

## Components

### Overview

Components are peices of data you add to [entites](#overview). These can hold tons of data or none at all depending on what fits best for your use case.

### Example Component

You can turn a rust struct or enum into a component by adding `#[derive(Component)]` above it. For example you could create a `Movement` component that holds infomation about how to move the entity.

```rs
#[derive(Component)]
struct Movement {
    horizontal: f32,
    jump: bool,
}
```

### Example Tag

A tag is just a normal component with the only diference being that it doesn't hold any data. Tags are most commonly used to mark entities and distinguish them from other entities. In this example we have a component called `Player Tag`, having this allows us to distinguish between our player entity and other entities like our enemies. The following is how you would define a tag.

```rs
#[derive(Component)]
struct PlayerTag;
```

## Systems

### Overview

Systems are what you use to implement your game logic. A system is just a normal rust function that can take a certain set of arguments. You then add the function to the bevy app when creating it.

### Querys

A query is what you use to read and write data to and from entities. For example if you wanted to get all entites with `Transform`, `Movement` and `PlayerTag` components you can use the following.

```rs
Query<(&mut Transform, &Movement), With<PlayerTag>>
```

This will give you edit and read access to the `Transform` component, read access to the `Movement` component and only select entites that also have a `PlayerTag` component.

### Resources

Resources are what you use to store data and can be accessed from any system.

### Example Movement System

For this example we will query the entity example from above and use the data in the `Movement` component to move the entity position.

```rs
fn move_system(
    mut query: Query<(&mut Transform, &Movement), With<PlayerTag>>
) {
    for (mut transform, movement) in query.iter_mut() {
        transform.translation.x += movement.horizontal;
    }
}
```
