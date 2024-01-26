use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use crate::bullet::Bullet;
use crate::game::Health;

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
    mut enemy_query: Query<(Entity, &Transform, &mut Health), With<Enemy>>,
    bullet_query: Query<(Entity, &Transform, &Bullet)>
) {
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
                }
            }
        }
    }
}