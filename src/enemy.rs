use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use crate::bullet::Bullet;
use crate::game::{Health, PlayerStats, RoundInfo};
use crate::level::Waypoints;

#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
    pub waypoint_id: usize,
    pub direction: Vec3,
}

impl Enemy {
    pub fn new(spd: f32) -> Enemy {
        return Enemy {
            speed: spd,
            waypoint_id: 0,
            direction: Vec3::ZERO,
        }
    }
}

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_enemy);
        app.add_systems(Update, bullet_collision);
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

fn bullet_collision(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &Transform, &mut Health), With<Enemy>>,
    bullet_query: Query<(Entity, &Transform, &Bullet)>,
    mut player_stats_query: Query<&mut PlayerStats>,
    mut round_info_query: Query<&mut RoundInfo>,
) {
    let mut player_stats = player_stats_query.get_single_mut().unwrap();
    let mut round_info = round_info_query.get_single_mut().unwrap();

    let e_size = Vec2::new(32.0, 32.0);
    let b_size = Vec2::new(24.0, 4.0);

    for (e_entity, enemy, mut health) in enemy_query.iter_mut() {
        for (b_entity, b_transform, bullet) in bullet_query.iter() {
            let collision = collide(enemy.translation, e_size, b_transform.translation, b_size);
            
            if let Some(collision) = collision {
                commands.entity(b_entity).despawn();

                health.lose(bullet.get_damage());
                if (health.get_health() <= 0) {
                    commands.entity(e_entity).despawn();
                    round_info.enemies_killed += 1;
                    player_stats.add_coins(10);
                    continue;
                }

                player_stats.add_coins(1);
            }
        }
    }
}