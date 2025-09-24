use bevy::ecs::system::Resource;

use crate::Wind;

#[derive(Resource)]
pub struct GameConfig {
    pub movement_speed: f32,
    pub slide: Option<SlideFactors>,
    pub breaking_timer: Option<f32>,
    pub wind: Option<Wind>,
    pub mirror: bool,
    pub tilt: Option<i8>,
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig {
            movement_speed: 200.,
            slide: None,
            breaking_timer: None,
            wind: None,
            mirror: false,
            tilt: None,
        }
    }
}
impl GameConfig {
    pub fn calculate_speed(&self, direction: f32, current_speed: f32, tilt_speed: f32) -> f32 {
        let mut speed = if let Some(slide_factor) = &self.slide {
            if direction == 0.0 {
                let speed = current_speed * slide_factor.deceleration;

                // Otherwise speed will diverge to 0 & player runs very slowly forever
                if speed.abs() > 5. {
                    speed
                } else {
                    0.
                }
            } else {
                let speed =
                    direction * (slide_factor.acceleration * self.movement_speed) + current_speed;
                speed.clamp(-self.movement_speed, self.movement_speed)
            }
        } else {
            direction * self.movement_speed
        };
        speed += if let Some(_) = &self.tilt {
            tilt_speed * 5.
        } else {
            0.
        };
        speed
    }
}

pub struct SlideFactors {
    pub acceleration: f32,
    pub deceleration: f32,
}
