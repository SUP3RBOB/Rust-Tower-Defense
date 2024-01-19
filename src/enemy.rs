use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    speed: f32,
    pub waypoint_id: usize,
}

impl Enemy {
    pub fn new(spd: f32) -> Enemy {
        return Enemy {
            speed: spd,
            waypoint_id: 0,
        }
    }
}