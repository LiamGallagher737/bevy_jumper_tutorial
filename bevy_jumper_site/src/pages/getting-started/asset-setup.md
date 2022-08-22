---
title: Setup Assets
description: Setting up our assets for use in our game
layout: ../../layouts/MainLayout.astro
---

## Downloading the Assets

For this tutorial I am going to be using an asset pack from [kenney.nl](https://kenney.nl), you can download the same asset pack [here](https://kenney.nl/assets/jumper-pack). It's not required that you use the same assets but it will make it easier to follow along if you do.

## Add Assets to Bevy

Bevy makes it really easy to import assets, for now all you need to do is create a folder called `assets` in your project and add your assets to it. If your using the same asset pack it should look something like this, I'm going to be using a diferent player character so I left out the `players` folder.

```
📦Project Folder
 ┣ 📂src
 ┃ ┗ 📜main.rs
 ┣ 📂assets
 ┃ ┣ 📂Background
 ┃ ┣ 📂Enemies
 ┃ ┣ 📂Enviroment
 ┃ ┣ 📂Items
 ┃ ┗ 📂Particles
 ┣ 📜Cargo.toml
 ┗ Other Files
```

Under the `Enemies` folder I'm going to sort each character into their own folder to tidy things up a bit, this is optional.

```
📦Enemies
 ┣ 📂Cloud
 ┣ 📂Fly Man
 ┣ 📂Spike Ball
 ┣ 📂Spike Man
 ┣ 📂Spring Man
 ┣ 📂Sun
 ┗ 📂Wing Man
```
