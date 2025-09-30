use bevy::ecs::system::Resource;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

#[derive(Resource)]
pub struct RandomProgressState {
    pub lives: u32,
    pub intermediate_score: u32,
    pub level_count: u32,
    pub rng: StdRng,
}

impl Default for RandomProgressState {
    fn default() -> Self {
        Self {
            lives: 3,
            intermediate_score: 0,
            level_count: 0,
            rng: StdRng::seed_from_u64(42),
        }
    }
}

impl RandomProgressState {
    pub fn lose_life(&mut self) {
        if self.lives > 0 {
            self.lives -= 1;
        }
    }

    pub fn add_score(&mut self, score: u32) {
        self.intermediate_score += score;
        self.level_count += 1;
    }

    pub fn reset(&mut self) {
        self.lives = 3;
        self.intermediate_score = 0;
        self.level_count = 0;
        self.rng = StdRng::seed_from_u64(42);
    }

    pub fn is_game_over(&self) -> bool {
        self.lives == 0
    }

    pub fn advance_rng(&mut self) {
        // Advance the RNG state to ensure different results
        // Generate a few random values to advance the state
        let _: u32 = self.rng.random();
        let _: u32 = self.rng.random();
        let _: u32 = self.rng.random();
    }
}
