use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use rand::Rng;
use crate::game::{GameTimer, EnemySpawner, RoundInfo, Health};
use crate::enemy::{self, Enemy, EnemyBundle};
use crate::resources::Images;

const ENEMY_SPAWN_RATE: f32 = 3.0;
pub type EnemyType = fn(Handle<Image>) -> EnemyBundle;
pub const ENEMY_TYPES: [EnemyType; 4] = [
    enemy::weak_enemy,
    enemy::fast_enemy,
    enemy::medium_enemy,
    enemy::strong_enemy,
];

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

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_points);
        app.add_systems(Update, spawn_enemies);
    }
}

fn create_points(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    images: Res<Images>
) {
    let mut ps: Vec<Vec3> = Vec::new();
    ps.push(Vec3::new(220.0, -84.0, 0.0));
    ps.push(Vec3::new(220.0, 150.0, 0.0));
    ps.push(Vec3::new(400.0, 150.0, 0.0));
    ps.push(Vec3::new(400.0, 350.0, 0.0));
    ps.push(Vec3::new(220.0, 350.0, 0.0));
    ps.push(Vec3::new(220.0, 550.0, 0.0));
    ps.push(Vec3::new(600.0, 550.0, 0.0));
    ps.push(Vec3::new(600.0, 350.0, 0.0));
    ps.push(Vec3::new(1050.0, 350.0, 0.0));
    ps.push(Vec3::new(1050.0, 100.0, 0.0));
    ps.push(Vec3::new(800.0, 100.0, 0.0));
    ps.push(Vec3::new(800.0, 600.0, 0.0));
    ps.push(Vec3::new(1380.0, 600.0, 0.0));

    let mut paths: Vec<Vec3> = Vec::new();
    paths.push(Vec3::new(2.4, 9.6, -1.0));
    paths.push(Vec3::new(8.0, 2.4, -1.0));
    paths.push(Vec3::new(2.4, 8.4, -1.0));
    paths.push(Vec3::new(8.0, 2.4, -1.0));
    paths.push(Vec3::new(2.4, 8.3, -1.0));
    paths.push(Vec3::new(14.3, 2.4, -1.0));
    paths.push(Vec3::new(2.4, 8.3, -1.0));
    paths.push(Vec3::new(16.5, 2.4, -1.0));
    paths.push(Vec3::new(2.4, 8.5, -1.0));
    paths.push(Vec3::new(10.2, 2.4, -1.0));
    paths.push(Vec3::new(2.4, 18.05, -1.0));
    paths.push(Vec3::new(16.5, 2.4, -1.0));

    let mut i: usize = 0;
    while (i < ps.len() - 1) {
        let mut t = Transform {
            translation: (ps[i] + ps[i + 1usize]) / 2.0,
            scale: paths[i],
            ..default()
        };
        t.translation.z = -1.0;
        
        commands.spawn((
            SpriteBundle {
                transform: t,
                texture: images.path.clone(),
                visibility: Visibility::Visible,
                ..Default::default()
            },
            EnemyPath
        ));

        i += 1usize;
    }

    for point in ps.iter() {
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(8.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE)),
            transform: Transform::from_translation(*point),
            ..default()
        });
    }

    commands.spawn(
        Waypoints {
            points: ps,
        }
    );
}

fn spawn_enemies(
    mut commands: Commands,
    mut timer_query: Query<&mut GameTimer, With<EnemySpawner>>,
    mut round_info: ResMut<RoundInfo>,
    time: Res<Time>,
    images: Res<Images>,
) {
    if let Ok(mut timer) = timer_query.get_single_mut() {
        if (round_info.enemies_spawned >= round_info.total_enemies || round_info.round_completed()) {
            timer.reset();
            return;
        }

        timer.add_time(time.delta_seconds());

        if (timer.get_time() >= round_info.spawn_rate()) {
            let enemy_images: [Handle<Image>; 4] = [
                images.enemy_regular(),
                images.square.clone_weak(),
                images.square.clone_weak(),
                images.square.clone_weak()
            ];
            
            let type_count = round_info.enemy_types_count();
            let num: usize = rand::thread_rng().gen_range(0usize..type_count);
            commands.spawn(ENEMY_TYPES[num](enemy_images[num].clone_weak()));
            
            round_info.enemies_spawned += 1;
            timer.reset();
        }
    }
}