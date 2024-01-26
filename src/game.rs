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
}

impl PlayerStats {
    pub fn new(amount: i32) -> PlayerStats {
        return PlayerStats {
            coins: amount,
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