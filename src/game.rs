use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::egui::load::SizedTexture;
use bevy_egui::egui::{Button, Pos2};
use bevy_egui::{egui, EguiContext, EguiUserTextures};
use crate::resources::Images;
use crate::tower::{DirectionalTower, Tower};

const TOWER_BUTTON_SIZE: [f32; 2] = [158.0, 40.0];

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, game_init);
        app.add_systems(Update, place_tower_range_view);
        app.add_systems(Update, game_ui);
        app.add_systems(Update, end_round);
    }
}

#[derive(Component)]
pub struct GameTimer {
    time: f32,
}

impl GameTimer {
    pub fn new(start_time: f32) -> GameTimer {
        return GameTimer {
            time: start_time,
        };
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
        };
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

#[derive(Resource)]
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

#[derive(Resource)]
pub struct RoundInfo {
    round: i32,
    round_completed: bool,
    max_enemies: i32,
    pub enemies_spawned: i32,
    pub enemies_killed: i32,
    pub total_enemies: i32,
    auto_start_round: bool,
    spawn_rate: f32,
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
            spawn_rate: 3.0
        };
    }

    pub fn new_round(&mut self) {
        self.round += 1;
        self.round_completed = false;
        self.total_enemies += 3;
        self.max_enemies = self.total_enemies;
        self.enemies_spawned = 0;
        self.enemies_killed = 0;
        
        if (self.spawn_rate > 0.4) {
            self.spawn_rate -= 0.25;
        }
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

    pub fn spawn_rate(&self) -> f32 {
        return self.spawn_rate;
    }

    pub fn set_spawn_rate(&mut self, spawn_rate: f32) {
        self.spawn_rate = spawn_rate;
    }
}

#[derive(Component)]
pub struct RangeView;

fn game_init(
    mut commands: Commands,
    images: Res<Images>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, -50.0),
        ..default()
    });

    commands.spawn((GameTimer::new(0.0), EnemySpawner));

    commands.insert_resource(PlayerStats::new(100));
    commands.insert_resource(RoundInfo::new());

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(64.0, 64.0, -0.5),
            texture: images.range_view.clone(),
            visibility: Visibility::Hidden,
            ..default()
        },
        RangeView
    ));
}

fn place_tower_range_view(
    mut range_view_query: Query<(&mut Transform, &RangeView, &mut Visibility), Without<Tower>>,
    tower_query: Query<(&Transform, &Tower), Without<RangeView>>,
) {
    let (mut r_transform, view, mut visibility) = range_view_query.get_single_mut().unwrap();

    for (transform, tower) in tower_query.iter() {
        if ((!tower.activated && !tower.is_selected()) || (tower.activated && tower.is_selected())) {
            r_transform.translation = transform.translation;
            r_transform.translation.z = -0.5;
            r_transform.scale = Vec3::new(tower.get_range() * 2.0 / 32.0, tower.get_range() * 2.0 / 32.0, 1.0);
            (*visibility) = Visibility::Visible;
            return;
        }
    }

    (*visibility) = Visibility::Hidden;
}

fn game_ui(
    mut contexts: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut commands: Commands,
    mut player_stats: ResMut<PlayerStats>,
    mut round_info: ResMut<RoundInfo>,
    mut range_view_query: Query<(&mut Transform, &mut Visibility), With<RangeView>>,
    mut tower_query: Query<&mut Tower>,
    mut egui_user_textures: ResMut<EguiUserTextures>,
    images: Res<Images>,
) {
    let (r_transform, mut r_visible) = range_view_query.get_single_mut().unwrap();

    if (player_stats.is_placing) {
        return;
    }

    let tower1_icon = SizedTexture::new(egui_user_textures.add_image(images.tower1.clone_weak()), [32.0, 32.0]);
    let tower2_icon = SizedTexture::new(egui_user_textures.add_image(images.tower2.clone_weak()), [32.0, 32.0]);
    let tower3_icon = SizedTexture::new(egui_user_textures.add_image(images.tower3.clone_weak()), [32.0, 32.0]);

    let Ok(mut ctx) = contexts.get_single_mut() else {
        return;
    };

    egui::Window::new("Game").default_pos(Pos2::new(4.0, 4.0)).show(ctx.get_mut(), |ui| {
        ui.label(format!("Round: {}", round_info.round));
        ui.label(format!("Coins: {}", player_stats.coins));

        if (round_info.round_completed()) {
            if (ui.button("New Round").clicked()) {
                round_info.new_round();
            }
        }
        
        ui.checkbox(&mut round_info.auto_start_round, "Auto Start New Round");

        if (ui.add_sized(TOWER_BUTTON_SIZE, Button::image_and_text(tower1_icon, "Tower 1 | 50 Coins")).clicked() && player_stats.get_coins() >= 50) {
            for mut tower in tower_query.iter_mut() {
                tower.set_selected(false);
            }

            player_stats.is_placing = true;

            let mut t = Transform::from_translation(Vec3::new(-16.0, -16.0, 3.0));
            t.scale = Vec3::new(2.0, 2.0, 2.0);

            commands.spawn((SpriteBundle {
                transform: t,
                texture: images.tower1.clone(),
                visibility: Visibility::Visible,
                ..default()
            },
                Tower::new(150.0, 0.8, 50),
                GameTimer::new(0.0))
            );

            (*r_visible) = Visibility::Visible;
        }

        if (ui.add_sized(TOWER_BUTTON_SIZE, Button::image_and_text(tower2_icon, "Tower 2 | 100 Coins")).clicked() && player_stats.get_coins() >= 100) {
            for mut tower in tower_query.iter_mut() {
                tower.set_selected(false);
            }

            player_stats.is_placing = true;

            let mut t = Transform::from_translation(Vec3::new(-16.0, -16.0, 3.0));
            t.scale = Vec3::new(2.0, 2.0, 2.0);

            commands.spawn((SpriteBundle {
                transform: t,
                texture: images.tower2.clone(),
                visibility: Visibility::Visible,
                ..default()
            },
                Tower::new(120.0, 1.2, 100),
                GameTimer::new(0.0),
                )
            );

            (*r_visible) = Visibility::Visible;
        }

        if (ui.add_sized(TOWER_BUTTON_SIZE, Button::image_and_text(tower3_icon, "Tower 3 | 75 Coins")).clicked() && player_stats.get_coins() >= 75) {
            for mut tower in tower_query.iter_mut() {
                tower.set_selected(false);
            }

            player_stats.is_placing = true;

            let mut t = Transform::from_translation(Vec3::new(-16.0, -16.0, 3.0));
            t.scale = Vec3::new(2.0, 2.0, 2.0);

            commands.spawn((SpriteBundle {
                transform: t,
                texture: images.tower3.clone(),
                visibility: Visibility::Visible,
                ..default()
            },
                Tower::new(110.0, 1.2, 75),
                GameTimer::new(0.0),
                DirectionalTower)
            );

            (*r_visible) = Visibility::Visible;
        }
    });
}

fn end_round(
    mut round_info: ResMut<RoundInfo>
) {
    if (round_info.enemies_spawned == round_info.enemies_killed 
        && round_info.enemies_killed >= round_info.total_enemies
        && !round_info.round_completed()) {
        round_info.round_completed = true;
    }

    if (round_info.round_completed && round_info.auto_start_round) {
        round_info.new_round();
    }
}