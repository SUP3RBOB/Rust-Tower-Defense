mod level;
mod enemy;
mod game;
mod tower;
mod bullet;

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use enemy::Enemy;
use game::Health;
use game::PlayerStats;
use game::RoundInfo;
use level::Waypoints;
use level::EnemyPath;
use game::GameTimer;
use game::EnemySpawner;

const ENEMY_SPAWN_RATE: f32 = 3.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_systems(Startup, game_init)
        .add_systems(Startup, create_points)
        .add_systems(Update, tower::place_tower)
        .add_systems(Update, spawn_enemies)
        .add_systems(Update, move_enemy)
        .add_systems(Update, tower::update_tower)
        .add_systems(Update, bullet::update_bullets)
        .add_systems(Update, enemy::bullet_collision)
        .add_systems(Update, game::game_ui)
        .run();
}

fn game_init(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, -1.0),
        ..default()
    });

    commands.spawn((GameTimer::new(0.0), EnemySpawner));
    commands.spawn(PlayerStats::new(100));
    commands.spawn(RoundInfo::new());
}

fn create_points(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
) {
    let mut ps: Vec<Vec3> = Vec::new();
    ps.push(Vec3::new(220.0, -84.0, 0.0));
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

    let mut paths: Vec<Vec3> = Vec::new();
    paths.push(Vec3::new(2.4, 9.6, 1.0));
    paths.push(Vec3::new(8.0, 2.4, 1.0));
    paths.push(Vec3::new(2.4, 8.4, 1.0));
    paths.push(Vec3::new(8.0, 2.4, 1.0));
    paths.push(Vec3::new(2.4, 8.3, 1.0));
    paths.push(Vec3::new(14.3, 2.4, 1.0));
    paths.push(Vec3::new(2.4, 8.3, 1.0));
    paths.push(Vec3::new(16.5, 2.4, 1.0));
    paths.push(Vec3::new(2.4, 8.5, 1.0));
    paths.push(Vec3::new(10.2, 2.4, 1.0));
    paths.push(Vec3::new(2.4, 18.05, 1.0));
    paths.push(Vec3::new(16.5, 2.4, 1.0));

    let mut i = 0;
    while (i < ps.len() - 1) {
        let mut t = Transform {
            translation: (ps[i] + ps[i + 1usize]) / 2.0,
            scale: paths[i],
            ..default()
        };
        t.translation.z = -1.0;
        
        commands.spawn((
            SpriteBundle {
                transform: t,
                texture: asset_server.load("sprites/path.png"),
                visibility: Visibility::Visible,
                ..Default::default()
            },
            EnemyPath
        ));

        i += 1usize;
    }

    for point in ps.iter() {
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(8.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE)),
            transform: Transform::from_translation(*point),
            ..default()
        });
    }

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
    round_info_query: Query<&RoundInfo>,
    time: Res<Time>,
) {
    if let Ok(mut timer) = timer_query.get_single_mut() {
        let round_info = round_info_query.get_single().unwrap();

        if (round_info.round_completed()) {
            return;
        }

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
                Health::new(30),
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