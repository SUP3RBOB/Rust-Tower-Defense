use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use crate::bullet::Bullet;

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

pub fn bullet_collision(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>
) {
    let e_size = Vec2::new(32.0, 32.0);
    let b_size = Vec2::new(24.0, 4.0);

    for (e_entity, enemy) in enemy_query.iter() {
        for (b_entity, bullet) in bullet_query.iter() {
            let collision = collide(enemy.translation, e_size, bullet.translation, b_size);
            
            if let Some(collision) = collision {
                commands.entity(e_entity).despawn();
                commands.entity(b_entity).despawn();
            }
        }
    }
}