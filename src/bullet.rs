use bevy::prelude::*;
use crate::game::GameTimer;

#[derive(Component)]
pub struct Bullet {
    damage: i32,
    direction: Vec3,
    speed: f32,
    lifetime: f32,
}

impl Bullet {
    pub fn new(dmg: i32, dir: Vec3, spd: f32, life: f32) -> Bullet {
        return Bullet {
            damage: dmg,
            direction: dir,
            speed: spd,
            lifetime: life,
        }
    }

    pub fn update(&mut self, transform: &mut Transform, delta: f32) {
        transform.translation += self.direction * self.speed * delta;
    }

    pub fn get_lifetime(&self) -> f32 {
        return self.lifetime;
    }

    pub fn get_damage(&self) -> i32 {
        return self.damage;
    }
}

pub struct BulletPlugin;
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_bullets);
    }
}

pub fn update_bullets(
    mut commands: Commands,
    mut bullet_query: Query<(&mut Bullet, &mut Transform, &mut GameTimer, Entity)>,
    time: Res<Time>
) {
    for (mut bullet, mut transform, mut timer, entity) in bullet_query.iter_mut() {
        bullet.update(&mut transform, time.delta_seconds());
        timer.add_time(time.delta_seconds());

        if (timer.get_time() >= bullet.get_lifetime()) {
            commands.entity(entity).despawn();
        }
    }
}