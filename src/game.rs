use bevy::prelude::*;

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

    pub fn add(&mut self, amount: i32) {
        self.health += amount;
    }

    pub fn lose(&mut self, amount: i32) {
        self.health -= amount;
    }
}