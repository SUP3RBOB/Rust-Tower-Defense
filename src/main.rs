#![allow(unused_parens)]

mod level;
mod enemy;
mod game;
mod tower;
mod bullet;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(EguiPlugin)
        .add_plugins(tower::TowerPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(bullet::BulletPlugin)
        .add_plugins(level::LevelPlugin)
        .add_plugins(game::GamePlugin)
        .run();
}