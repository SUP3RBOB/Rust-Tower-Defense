use bevy::prelude::*;

use crate::enemy::Enemy;

#[derive(Component)]
pub struct Tower {
    range: f32,
    direction: Vec3,
}

impl Tower {
    pub fn get_closest_in_range(&self, player_pos: Vec3, transforms: &Query<&Transform, With<Enemy>>) -> Vec3 {
        let mut closest = player_pos;

        for transform in transforms.iter() {
            let prev_dist = Vec3::distance(player_pos, closest);
            let dist = Vec3::distance(player_pos, transform.translation);

            if (dist > self.range) {
                continue;
            }

            if (prev_dist > dist) {
                closest = transform.translation;
            }
        }

        return closest;
    }

    pub fn rotate_towards(&self, transform: &mut Transform, point: Vec3) {
        let difference = point - transform.translation;
        let angle = f32::to_degrees(f32::atan2(difference.y, difference.x));
        transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, angle);
    }

    pub fn set_direction(&mut self, dir: Vec3) {
        self.direction = dir;
    }
}

pub fn place_tower(
    mut commands: Commands,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mouse: Res<Input<MouseButton>>,
    asset_server: Res<AssetServer>,
) {
    if (mouse.just_pressed(MouseButton::Left)) {
        let window = windows.single();
        let (camera, camera_transform) = camera_query.single();

        if let Some(world_position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)) {

            let spawn_position = Vec3::new(world_position.x, world_position.y, 0.0);
            commands.spawn(SpriteBundle {
                transform: Transform::from_translation(spawn_position),
                texture: asset_server.load("sprites/tower.png"),
                visibility: Visibility::Visible,
                ..default()
            });
        }
    }
}

pub fn update_tower(
    mut commands: Commands,
    mut tower_query: Query<(&mut Tower, &mut Transform)>,
    enemy_query: Query<&Transform, With<Enemy>>
) {
    for (mut tower, mut transform) in tower_query.iter_mut() {
        let closest = tower.get_closest_in_range(transform.translation, &enemy_query);
        tower.set_direction(Vec3::normalize(closest - transform.translation));
        tower.rotate_towards(&mut transform, closest);
        
    }
}