---
title: Rust Setup
description: Setting up Rust so we can use it to create our game
layout: ../../layouts/MainLayout.astro
---

## Installing Rust

First, download the Rust installer from the [official rust website](https://www.rust-lang.org/learn/get-started). Run the installer and follow the instructions, it ay prompt you to install some 3rd party tools, if it does follow along with the steps it shows. You may have to restart your computer to complete installaton.

## Creating a Rust Project

The easiest way to create a new Rust project is to open an empty folder in a terminal and run `cargo init`, this will create a new Rust binary application.

## Project Structure

After runnng `cargo init` your folder should look like this.

```
📦Your Folder
 ┣ 📂src
 ┃ ┗ 📜main.rs
 ┣ 📜.gitignore
 ┣ 📜Cargo.toml
```

## Check it's working

To ensure everything is working properly enter `cargo run` in the terminal and after a bit of waiting `Hello, world!` should appear in the termnal. Your project folder should also have updated to look like the following.

```
📦Your Folder
 ┣ 📂src
 ┃ ┗ 📜main.rs
 ┣ 📂target
 ┃ ┗ A ton of files
 ┣ 📜.gitignore
 ┣ 📜Cargo.toml
 ┣ 📜Cargo.lock
```

The target folder is where your application files and data is outputted when you run or build it.
