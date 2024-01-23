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