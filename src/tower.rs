use bevy::prelude::*;

use crate::enemy::Enemy;
use crate::bullet::Bullet;
use crate::game::GameTimer;
use crate::level::EnemyPath;
use crate::game::PlayerStats;

#[derive(Component)]
pub struct Tower {
    activated: bool,
    range: f32,
    direction: Vec3,
    rate_of_fire: f32,
    cost: i32,
}

impl Tower {
    pub fn new(range_: f32, rof: f32, price: i32) -> Tower {
        return Tower {
            activated: false,
            range: range_,
            direction: Vec3::ZERO,
            rate_of_fire: rof,
            cost: price,
        }
    }

    pub fn closest_in_range(&self, player_pos: Vec3, points: &Vec<Vec3>, out_closest: &mut Vec3) -> bool {
        let mut closest = Vec3::new(-2000.0, -2000.0, 0.0);

        for point in points.iter() {
            let prev_dist = Vec3::distance(player_pos, closest);
            let dist = Vec3::distance(player_pos, (*point));

            if (dist > self.range) {
                continue;
            }

            if (prev_dist > dist) {
                closest = (*point);
            }
        }

        (*out_closest) = closest;
        return (*out_closest) != Vec3::new(-2000.0, -2000.0, 0.0);
    }

    pub fn rotate_towards(&self, transform: &mut Transform, point: Vec3) {
        let difference = point - transform.translation;
        let angle = f32::atan2(difference.y, difference.x);
        transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, angle);
    }

    pub fn set_direction(&mut self, dir: Vec3) {
        self.direction = dir;
    }

    pub fn acitvate(&mut self, activate: bool) {
        self.activated = activate;
    }
}

pub fn place_tower(
    mut commands: Commands,
    windows: Query<&Window>,
    path_query: Query<(&Transform, &EnemyPath), Without<Tower>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mouse: Res<Input<MouseButton>>,
    mut tower_query: Query<(Entity, &mut Tower, &mut Transform)>,
    mut player_stats_query: Query<&mut PlayerStats>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_query.single();
    let mut player_stats = player_stats_query.get_single_mut().unwrap();

    for (entity, mut tower, mut transform) in tower_query.iter_mut() {
        if (tower.activated) {
            continue;
        }

        if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)) {
        
            transform.translation = Vec3::new(world_position.x, world_position.y, 3.0);
        }

        if (mouse.pressed(MouseButton::Left)) {
            tower.acitvate(true);
            player_stats.is_placing = false;
            player_stats.lose_coins(tower.cost);
        }

        if (mouse.pressed(MouseButton::Right)) {
            commands.entity(entity).despawn();
            player_stats.is_placing = false;
        }
    }
}

pub fn update_tower(
    mut commands: Commands,
    mut tower_query: Query<(&mut Tower, &mut Transform, &mut GameTimer)>,
    enemy_query: Query<(&Transform, &Enemy), Without<Tower>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>
) {
    let mut points: Vec<Vec3> = Vec::new();
    for (transform, enemy) in enemy_query.iter() {
        points.push(transform.translation + (enemy.direction * 32.0));
    }

    for (mut tower, mut transform, mut timer) in tower_query.iter_mut() {
        if (!tower.activated) {
            continue;
        }
        
        let mut closest = Vec3::new(-2000.0, -2000.0, 0.0);
        let has_closest = tower.closest_in_range(transform.translation, &points, &mut closest);
        //println!("{}, {}, {}", closest.x, closest.y, closest.z);

        if (!has_closest) {
            return;
        }

        tower.set_direction(Vec3::normalize(closest - transform.translation));
        tower.rotate_towards(&mut transform, closest);

        timer.add_time(time.delta_seconds());
        if (timer.get_time() >= tower.rate_of_fire) {
            let mut bullet = Transform::from_translation(transform.translation);
            bullet.rotation = transform.rotation;

            commands.spawn((
                Bullet::new(10, tower.direction, 550.0, 2.0),
                SpriteBundle {
                    transform: bullet,
                    texture: asset_server.load("sprites/bullet.png"),
                    visibility: Visibility::Visible,
                    ..default()
                },
                GameTimer::new(0.0)
            ));
            timer.reset();
        }
    }
}