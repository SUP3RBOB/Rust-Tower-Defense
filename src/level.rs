use bevy::prelude::*;

#[derive(Component)]
pub struct Waypoints {
    pub points: Vec<Vec3>,
}

#[derive(Component)]
pub struct EnemyPath;

impl EnemyPath {
    fn point_in_path(point: Vec3, transform: &Transform) {
        
    }
}