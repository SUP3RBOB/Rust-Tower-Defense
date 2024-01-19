use bevy::prelude::*;

#[derive(Component)]
pub struct Waypoints {
    pub points: Vec<Vec3>,
}