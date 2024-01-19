mod level;
mod enemy;
mod game;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use enemy::Enemy;
use level::Waypoints;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_enemy)
        .add_systems(Startup, create_points)
        .add_systems(Update, move_enemy)
        .run();
}

fn create_points(mut commands: Commands) {
    let mut ps: Vec<Vec3> = Vec::new();
    ps.push(Vec3::new(64.0, 64.0, 0.0));
    ps.push(Vec3::new(256.0, 128.0, 0.0));
    ps.push(Vec3::new(800.0, 45.0, 0.0));
    ps.push(Vec3::new(428.0, 427.0, 0.0));
    commands.spawn(
        Waypoints {
            points: ps,
        }
    );
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, -1.0),
        ..default()
    });
;}

fn spawn_enemy(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Enemy::new(10.0),
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            texture: asset_server.load("sprites/square.png"),
            visibility: Visibility::Visible,
            ..Default::default()
        },
    ));
}

fn move_enemy(
    mut commands: Commands,
    mut query: Query<(&mut Enemy, &mut Transform, Entity)>,
    mut waypoints: Query<&Waypoints>,
    time: Res<Time>,
) {
    if let Ok(mut enemy) = query.get_single_mut() {
        let dir = Vec3::normalize(waypoints.single().points[enemy.0.waypoint_id] - enemy.1.translation);
        let dist = Vec3::distance(enemy.1.translation, waypoints.single().points[enemy.0.waypoint_id]);

        if (dist <= 1.0) {
            if (enemy.0.waypoint_id < waypoints.single().points.len() - 1) {
                enemy.0.waypoint_id += 1;
            } else {
                commands.entity(enemy.2).despawn();
            }
        } else {
            enemy.1.translation += dir * 150.0 * time.delta_seconds();
        }

        println!("Enemy position: {}, {}, {} at waypoint {}", enemy.1.translation.x, enemy.1.translation.y, enemy.1.translation.z, enemy.0.waypoint_id);
    }
}