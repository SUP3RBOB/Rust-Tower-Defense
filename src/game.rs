use bevy::prelude::*;

#[derive(Component)]
pub struct Timer {
    timer: f32,
}

impl Timer {
    pub fn new(start_time: f32) -> Timer {
        return Timer {
            timer: 0.0,
        }
    }

    pub fn add_time(&mut self, delta: f32) {
        self.timer += delta;
    }

    pub fn reset_timer(&mut self) {
        self.timer = 0.0;
    }
}