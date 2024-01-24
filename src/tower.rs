use bevy::prelude::*;

use crate::enemy::Enemy;
use crate::bullet::Bullet;

#[derive(Component)]
pub struct Tower {
    range: f32,
    direction: Vec3,
}

impl Tower {
    pub fn new(range_: f32) -> Tower {
        return Tower {
            range: range_,
            direction: Vec3::ZERO,
        }
    }

    pub fn closest_in_range(&self, player_pos: Vec3, points: &Vec<Vec3>, out_closest: &mut Vec3) -> bool {
        let mut closest = Vec3::ZERO;

        for point in points.iter() {
            let prev_dist = Vec3::distance(player_pos, closest);
            let dist = Vec3::distance(player_pos, (*point));

            if (dist > self.range) {
                continue;
            }

            if (prev_dist > dist) {
                closest = (*point);
            }
        }

        (*out_closest) = closest;
        return (*out_closest) != Vec3::ZERO;
    }

    pub fn rotate_towards(&self, transform: &mut Transform, point: Vec3) {
        let difference = point - transform.translation;
        let angle = f32::atan2(difference.y, difference.x);
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
            commands.spawn((SpriteBundle {
                transform: Transform::from_translation(spawn_position),
                texture: asset_server.load("sprites/tower.png"),
                visibility: Visibility::Visible,
                ..default()
            },
            Tower::new(300.0))
            );
        }
    }
}

pub fn update_tower(
    mut commands: Commands,
    mut set: ParamSet <(
        Query<(&mut Tower, &mut Transform)>,
        Query<&Transform, With<Enemy>>,
    )>
) {
    let mut points: Vec<Vec3> = Vec::new();
    for transform in set.p1().iter() {
        points.push(transform.translation);
    }

    for (mut tower, mut transform) in set.p0().iter_mut() {
        let mut closest = Vec3::ZERO;
        let has_closest = tower.closest_in_range(transform.translation, &points, &mut closest);
        //println!("{}, {}, {}", closest.x, closest.y, closest.z);
        tower.set_direction(Vec3::normalize(closest - transform.translation));

        if (has_closest) {
            tower.rotate_towards(&mut transform, closest);
        }
    }
}