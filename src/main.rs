mod level;
mod enemy;

use bevy::prelude::*;
use enemy::Enemy;
use level::Waypoints;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_enemy)
        .add_systems(Startup, create_points)
        .add_systems(Update, move_enemy)
        .run();
}

fn create_points(mut commands: Commands) {
    let mut ps: Vec<Vec3> = Vec::new();
    ps.push(Vec3::new(64.0, 64.0, 0.0));
    ps.push(Vec3::new(128.0, 128.0, 0.0));
    ps.push(Vec3::new(192.0, 192.0, 0.0));
    ps.push(Vec3::new(256.0, 256.0, 0.0));
    commands.spawn(
        Waypoints {
            points: ps,
        }
    );
}

fn spawn_enemy(mut commands: Commands) {
    commands.spawn((
        Enemy::new(10.0),
        SpatialBundle {
            transform: Transform::from_scale(Vec3::splat(1.0)),
            visibility: Visibility::Visible,
            ..Default::default()
        },
    ));
}

fn move_enemy(mut query: Query<(&mut Enemy, &mut Transform)>, mut waypoints: Query<&Waypoints>) {
    if let Ok(mut waypoint) = waypoints.get_single_mut() {

    }

    if let Ok(mut enemy) = query.get_single_mut() {
        let dir = Vec3::normalize(waypoints.single().points[0] - enemy.1.translation);
        enemy.1.translation += dir;
        println!("Enemy position: {}, {}, {}", enemy.1.translation.x, enemy.1.translation.y, enemy.1.translation.z)
    }
}