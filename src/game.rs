use std::collections::btree_map::Range;

use bevy::{prelude::*, transform};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use crate::tower::Tower;

#[derive(Component)]
pub struct GameTimer {
    time: f32,
}

impl GameTimer {
    pub fn new(start_time: f32) -> GameTimer {
        return GameTimer {
            time: start_time,
        }
    }

    pub fn get_time(&self) -> f32 {
        return self.time;
    }

    pub fn add_time(&mut self, delta: f32) {
        self.time += delta;
    }

    pub fn reset(&mut self) {
        self.time = 0.0;
    }
}

#[derive(Component)]
pub struct EnemySpawner;

#[derive(Component)]
pub struct Health {
    health: i32,
    max_health: i32,
}

impl Health {
    pub fn new(max_hp: i32) -> Health {
        return Health {
            health: max_hp,
            max_health: max_hp
        }
    }

    pub fn get_health(&self) -> i32 {
        return self.health;
    }

    pub fn add(&mut self, amount: i32) {
        self.health += amount;
    }

    pub fn lose(&mut self, amount: i32) {
        self.health -= amount;
    }
}

#[derive(Component)]
pub struct PlayerStats {
    coins: i32,
    pub is_placing: bool
}

impl PlayerStats {
    pub fn new(amount: i32) -> PlayerStats {
        return PlayerStats {
            coins: amount,
            is_placing: false
        };
    }

    pub fn get_coins(&self) -> i32 {
        return self.coins;
    }

    pub fn set_coins(&mut self, amount: i32) {
        self.coins = amount;
    }

    pub fn add_coins(&mut self, amount: i32) {
        self.coins += amount;
    }

    pub fn lose_coins(&mut self, amount: i32) {
        self.coins -= amount;
    }

}

#[derive(Component)]
pub struct RoundInfo {
    round: i32,
    round_completed: bool,
    max_enemies: i32,
    enemies_spawned: i32,
    enemies_killed: i32,
    total_enemies: i32,
    auto_start_round: bool,
}

impl RoundInfo {
    pub fn new() -> RoundInfo {
        return RoundInfo {
            round: 0,
            round_completed: true,
            max_enemies: 0,
            enemies_spawned: 0,
            enemies_killed: 0,
            total_enemies: 3,
            auto_start_round: false,
        };
    }

    pub fn new_round(&mut self) {
        self.round += 1;
        self.round_completed = false;
        self.total_enemies += self.round * 2;
        self.max_enemies = self.total_enemies;
        self.enemies_spawned = 0;
        self.enemies_killed = 0;
    }

    pub fn round_completed(&self) -> bool {
        return self.round_completed;
    }

    pub fn get_round_auto_start(&self) -> bool {
        return self.auto_start_round;
    }

    pub fn set_round_auto_start(&mut self, start: bool) {
        self.auto_start_round = start;
    }
}

#[derive(Component)]
pub struct RangeView;

pub fn place_tower_range_view(
    mut range_view_query: Query<(&mut Transform, &RangeView), Without<Tower>>,
    tower_query: Query<(&Transform, &Tower), Without<RangeView>>,
) {
    let (mut r_transform, view) = range_view_query.get_single_mut().unwrap();

    for (transform, tower) in tower_query.iter() {
        if (tower.activated) {
            continue;
        }

        r_transform.translation = transform.translation;
        r_transform.translation.z = -0.5;
        r_transform.scale = Vec3::new(tower.get_range() / 32.0, tower.get_range() / 32.0, 1.0);
    }
}

pub fn game_ui(
    mut contexts: EguiContexts,
    mut commands: Commands,
    mut player_stats_query: Query<&mut PlayerStats>,
    mut round_info_query: Query<&mut RoundInfo>,
    mut range_view_query: Query<(&mut Transform, &mut Visibility), With<RangeView>>,
    asset_server: Res<AssetServer>
) {
    let mut player_info = player_stats_query.get_single_mut().unwrap();
    let mut round_info = round_info_query.get_single_mut().unwrap();
    let (mut r_transform, mut r_visible) = range_view_query.get_single_mut().unwrap();

    if (player_info.is_placing) {
        return;
    }

    egui::Window::new("Game").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("Round: {}", round_info.round));
        ui.label(format!("Coins: {}", player_info.coins));

        if (round_info.round_completed()) {
            if (ui.button("New Round").clicked()) {
                round_info.new_round();
            }
        }

        if (ui.button("Place Tower").clicked() && player_info.get_coins() >= 50) {
            player_info.is_placing = true;

            commands.spawn((SpriteBundle {
                transform: Transform::from_translation(Vec3::new(-16.0, -16.0, 3.0)),
                texture: asset_server.load("sprites/tower.png"),
                visibility: Visibility::Visible,
                ..default()
            },
            Tower::new(300.0, 0.8, 50),
            GameTimer::new(0.0))
            );

            r_transform.scale = Vec3::new(300.0 / 32.0, 300.0 / 32.0, 1.0);
            (*r_visible) = Visibility::Visible;
        }
    });
}