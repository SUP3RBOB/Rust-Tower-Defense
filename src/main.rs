mod level;
mod enemy;
mod game;
mod tower;
mod bullet;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use enemy::Enemy;
use level::Waypoints;
use game::GameTimer;
use game::EnemySpawner;
use bullet::Bullet;

const ENEMY_SPAWN_RATE: f32 = 3.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, game_init)
        .add_systems(Startup, create_points)
        .add_systems(Update, tower::place_tower)
        .add_systems(Update, spawn_enemies)
        .add_systems(Update, move_enemy)
        .add_systems(Update, tower::update_tower)
        .add_systems(Update, bullet::update_bullets)
        .add_systems(Update, enemy::bullet_collision)
        .run();
}

fn game_init(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, -1.0),
        ..default()
    });

    commands.spawn((GameTimer::new(0.0), EnemySpawner));
}

fn create_points(mut commands: Commands) {
    let mut ps: Vec<Vec3> = Vec::new();
    ps.push(Vec3::new(220.0, 150.0, 0.0));
    ps.push(Vec3::new(400.0, 150.0, 0.0));
    ps.push(Vec3::new(400.0, 350.0, 0.0));
    ps.push(Vec3::new(220.0, 350.0, 0.0));
    ps.push(Vec3::new(220.0, 550.0, 0.0));
    ps.push(Vec3::new(600.0, 550.0, 0.0));
    ps.push(Vec3::new(600.0, 350.0, 0.0));
    ps.push(Vec3::new(1050.0, 350.0, 0.0));
    ps.push(Vec3::new(1050.0, 100.0, 0.0));
    ps.push(Vec3::new(800.0, 100.0, 0.0));
    ps.push(Vec3::new(800.0, 600.0, 0.0));
    ps.push(Vec3::new(1380.0, 600.0, 0.0));
    commands.spawn(
        Waypoints {
            points: ps,
        }
    );
}

fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut timer_query: Query<&mut GameTimer, With<EnemySpawner>>,
    time: Res<Time>,
) {
    if let Ok(mut timer) = timer_query.get_single_mut() {
        timer.add_time(time.delta_seconds());

        if (timer.get_time() >= ENEMY_SPAWN_RATE) {
            commands.spawn((
                Enemy::new(150.0),
                SpriteBundle {
                    transform: Transform::from_xyz(220.0, -84.0, 0.0),
                    texture: asset_server.load("sprites/square.png"),
                    visibility: Visibility::Visible,
                    ..Default::default()
                },
            ));
            timer.reset();
        }
    }
}

fn move_enemy(
    mut commands: Commands,
    mut query: Query<(&mut Enemy, &mut Transform, Entity)>,
    waypoints: Query<&Waypoints>,
    time: Res<Time>,
) {
    for (mut enemy, mut transform, entity) in query.iter_mut() {
        let dir = Vec3::normalize(waypoints.single().points[enemy.waypoint_id] - transform.translation);
        let dist = Vec3::distance(transform.translation, waypoints.single().points[enemy.waypoint_id]);

        if (dist <= 6.0) {
            if (enemy.waypoint_id < waypoints.single().points.len() - 1usize) {
                enemy.waypoint_id += 1;
            } else {
                commands.entity(entity).despawn();
            }
        } else {
            transform.translation += dir * enemy.speed * time.delta_seconds();
            enemy.direction = dir;
        }

        //println!("Enemy position: {}, {}, {} at waypoint {}", enemy.1.translation.x, enemy.1.translation.y, enemy.1.translation.z, enemy.0.waypoint_id);
    }
}