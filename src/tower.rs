use bevy::prelude::*;

#[derive(Component)]
pub struct Tower {
    range: f32,
    direction: Vec3,
}

impl Tower {
    pub fn get_closest_in_range(&self, player_pos: Vec3, transforms: &Query<&Transform>) -> Vec3 {
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