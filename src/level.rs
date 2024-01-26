use bevy::prelude::*;

#[derive(Component)]
pub struct Waypoints {
    pub points: Vec<Vec3>,
}

#[derive(Component)]
pub struct EnemyPath;

impl EnemyPath {
    pub fn point_in_path(point: Vec2, transform: &Transform) -> bool {
        let width = 32.0 * transform.scale.x;
        let height = 32.0 * transform.scale.y;
        
        let top = transform.translation.y + height / 2.0;
        let bottom = transform.translation.y - height / 2.0;
        let right = transform.translation.x + width / 2.0;
        let left = transform.translation.x - width / 2.0;
        
        if (point.x >= left && point.x <= right && point.y <= top && point.y >= bottom) {
            return true;
        }

        return false;
    }
}