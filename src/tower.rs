use bevy::prelude::*;
use bevy_egui::egui::Pos2;
use bevy_egui::{egui, EguiContexts};

use crate::enemy::Enemy;
use crate::bullet::Bullet;
use crate::game::{GameTimer, RangeView};
use crate::level::EnemyPath;
use crate::game::PlayerStats;
use crate::resources::Images;

#[derive(Component)]
pub struct RegularTower;

#[derive(Component)]
pub struct DirectionalTower;

#[derive(Component)]
pub struct RocketTower;

#[derive(Component)]
pub struct Tower {
    pub activated: bool,
    range: f32,
    direction: Vec3,
    rate_of_fire: f32,
    cost: i32,
    selected: bool,
    level: i32
}

impl Tower {
    pub fn new(range_: f32, rof: f32, price: i32) -> Tower {
        return Tower {
            activated: false,
            range: range_,
            direction: Vec3::ZERO,
            rate_of_fire: rof,
            cost: price,
            selected: false,
            level: 1,
        }
    }

    pub fn closest_in_range(&self, player_pos: Vec3, points: &Vec<Vec3>, out_closest: &mut Vec3) -> bool {
        let mut closest = Vec3::ZERO;
        let mut min_distance = f32::INFINITY;

        for point in points.iter() {
            let dist = Vec3::distance(player_pos, (*point));

            if (dist > self.range) {
                continue;
            }

            if (dist < min_distance) {
                min_distance = dist;
                closest = (*point);
            }
        }

        (*out_closest) = closest;
        return min_distance != f32::INFINITY;
    }

    pub fn rotate_towards(&self, transform: &mut Transform, point: Vec3) {
        let difference = point - transform.translation;
        let angle = f32::atan2(difference.y, difference.x);
        transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, angle);
    }

    pub fn set_direction(&mut self, dir: Vec3) {
        self.direction = dir;
    }

    pub fn acitvate(&mut self, activate: bool) {
        self.activated = activate;
    }

    pub fn get_range(&self) -> f32 {
        return self.range;
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    pub fn is_selected(&self) -> bool {
        return self.selected;
    }

    pub fn clicked(&self, point: Vec2, transform: &Transform) -> bool {
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

    pub fn upgrade(&mut self, range: f32, rate_of_fire: f32) {
        self.range += range;
        
        if (self.rate_of_fire >= 0.1) {
            self.rate_of_fire -= rate_of_fire;
        }
    }
}

const DIRECTIONS: [Vec3; 8] = [
    Vec3::new(1.0, 0.0, 0.0),
    Vec3::new(0.707106781186548f32, 0.707106781186548f32, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    Vec3::new(-0.707106781186548f32, 0.707106781186548f32, 0.0),
    Vec3::new(-1.0, 0.0, 0.0),
    Vec3::new(-0.707106781186548f32, -0.707106781186548f32, 0.0),
    Vec3::new(0.0, -1.0, 0.0),
    Vec3::new(0.707106781186548f32, -0.707106781186548f32, 0.0)
];

pub struct TowerPlugin;
impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, place_tower);
        app.add_systems(Update, update_tower);
        app.add_systems(Update, update_directional_tower);
        app.add_systems(Update, upgrade_tower);
    }
}

fn place_tower(
    mut commands: Commands,
    windows: Query<&Window>,
    path_query: Query<(&Transform, &EnemyPath), Without<Tower>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mouse: Res<Input<MouseButton>>,
    mut tower_query: Query<(Entity, &mut Tower, &mut Transform)>,
    mut player_stats_query: Query<&mut PlayerStats>,
    mut range_view_query: Query<&mut Visibility, With<RangeView>>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_query.single();
    let mut player_stats = player_stats_query.get_single_mut().unwrap();
    let mut range_visibility = range_view_query.get_single_mut().unwrap();

    for (entity, mut tower, mut transform) in tower_query.iter_mut() {
        if (tower.activated) {
            continue;
        }

        if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)) {
        
            transform.translation = Vec3::new(world_position.x, world_position.y, 3.0);
            
            let mut in_path = false;
            for (path_trans, enemy_path) in path_query.iter() {
                in_path = EnemyPath::point_in_path(world_position, path_trans);
                if (in_path) {
                    break;
                }
            }

            (*range_visibility) = Visibility::Visible;

            if (mouse.just_released(MouseButton::Left)) {
                if (in_path) {
                    return;
                }

                tower.acitvate(true);
                player_stats.is_placing = false;
                player_stats.lose_coins(tower.cost);
                (*range_visibility) = Visibility::Hidden;
            }
    
            if (mouse.pressed(MouseButton::Right)) {
                commands.entity(entity).despawn();
                player_stats.is_placing = false;
                (*range_visibility) = Visibility::Hidden;
            }
        }
    }
}

fn update_tower(
    mut commands: Commands,
    mut tower_query: Query<(&mut Tower, &mut Transform, &mut GameTimer), Without<DirectionalTower>>,
    enemy_query: Query<(&Transform, &Enemy), Without<Tower>>,
    time: Res<Time>,
    images: Res<Images>
) {
    let mut points: Vec<Vec3> = Vec::new();
    for (transform, enemy) in enemy_query.iter() {
        points.push(transform.translation + (enemy.direction * 32.0));
    }

    for (mut tower, mut transform, mut timer) in tower_query.iter_mut() {
        if (!tower.activated) {
            continue;
        }
        
        let mut closest = Vec3::ZERO;
        let has_closest: bool = tower.closest_in_range(transform.translation, &points, &mut closest);
        //println!("{}, {}, {}", closest.x, closest.y, closest.z);

        if (!has_closest) {
            continue;
        }

        tower.set_direction(Vec3::normalize(closest - transform.translation));
        tower.rotate_towards(&mut transform, closest);

        timer.add_time(time.delta_seconds());
        if (timer.get_time() >= tower.rate_of_fire) {
            let mut bullet = Transform::from_translation(transform.translation);
            bullet.rotation = transform.rotation;

            commands.spawn((
                Bullet::new(10, tower.direction, 550.0, 2.0),
                SpriteBundle {
                    transform: bullet,
                    texture: images.bullet.clone(),
                    visibility: Visibility::Visible,
                    ..default()
                },
                GameTimer::new(0.0)
            ));
            timer.reset();
        }
    }
}

fn update_directional_tower(
    mut commands: Commands,
    time: Res<Time>,
    mut tower_query: Query<(&mut Tower, &mut Transform, &mut GameTimer), With<DirectionalTower>>,
    enemy_query: Query<(&Transform, &Enemy), Without<Tower>>,
    images: Res<Images>,
) {
    let mut points: Vec<Vec3> = Vec::new();
    for (transform, enemy) in enemy_query.iter() {
        points.push(transform.translation + (enemy.direction * 32.0));
    }

    for (mut tower, mut transform, mut timer) in tower_query.iter_mut() {
        if (!tower.activated) {
            continue;
        }
        
        let mut closest = Vec3::ZERO;
        let has_closest: bool = tower.closest_in_range(transform.translation, &points, &mut closest);
        //println!("{}, {}, {}", closest.x, closest.y, closest.z);

        if (!has_closest) {
            continue;
        }

        tower.set_direction(Vec3::normalize(closest - transform.translation));
        tower.rotate_towards(&mut transform, closest);

        timer.add_time(time.delta_seconds());
        if (timer.get_time() >= tower.rate_of_fire) {
            let mut bullet_t = Transform::from_translation(transform.translation);

            for dir in DIRECTIONS {
                let angle = f32::atan2(dir.y, dir.x);
                bullet_t.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, angle);

                commands.spawn((
                    Bullet::new(7, dir, 550.0, 1.75),
                    SpriteBundle {
                        transform: bullet_t,
                        texture: images.bullet.clone(),
                        visibility: Visibility::Visible,
                        ..default()
                    },
                    GameTimer::new(0.0)
                ));
                timer.reset();
            }
        }
    }
}

fn upgrade_tower(
    mut tower_query: Query<(&mut Tower, &Transform)>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mouse: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    mut range_view_query: Query<&mut Visibility, With<RangeView>>,
    mut contexts: EguiContexts,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_query.single();
    let mut range_view = range_view_query.get_single_mut().unwrap();

    if (mouse.just_released(MouseButton::Left) && !contexts.ctx_mut().is_pointer_over_area()) {
        if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)) {

            for (mut tower, transform) in tower_query.iter_mut() {
                tower.selected = false;
            }

            for (mut tower, transform) in tower_query.iter_mut() {
                if (tower.activated && !tower.selected && tower.clicked(world_position, &transform)) {
                    tower.selected = true;
                    break;
                }
                
                (*range_view) = Visibility::Hidden;
            }
        }
    }

    for (tower, transform) in tower_query.iter() {
        if (tower.selected) {
            egui::Window::new("Tower").default_pos(Pos2::new(1280.0, 720.0)).show(contexts.ctx_mut(), |ui| {
                ui.label("Level: 1");
                ui.label(format!("Rate of Fire: {} seconds", tower.rate_of_fire));
                if (ui.button("Upgrade Tower (20 Coins)").clicked()) {
                    println!("Tower upgraded!");
                }
            });
            return;
        }
    }
}