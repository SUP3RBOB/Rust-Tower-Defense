use bevy::prelude::*;

use crate::game::GameTimer;

#[derive(Component)]
pub struct Explosion {
    radius: f32,
    lifetime: f32,
    damaged: bool
}

impl Explosion {
    pub fn new(radius: f32, lifetime: f32) -> Explosion {
        return Explosion {
            radius,
            lifetime,
            damaged: false
        };
    }
}

pub struct ExplosionPlugin;
impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_explosions);
    }
}

fn update_explosions(
    mut explosion_query: Query<(Entity, &Explosion, &mut GameTimer)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, explosion, mut timer) in explosion_query.iter_mut() {
        timer.add_time(time.delta_seconds());

        if (timer.get_time() >= explosion.lifetime) {
            commands.entity(entity).despawn();
        }
    }
}