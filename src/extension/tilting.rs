use std::f32::consts::PI;

use bevy::prelude::*;
use rand::Rng;

use crate::GameConfig;

#[derive(Component)]
pub struct CameraTilt {
    pub target_rotation: f32,
    pub current_rotation: f32,
    pub rotation_speed: f32,
}
impl CameraTilt {
    pub fn tilt_speed_by_current_rotation(&self) -> f32 {
        self.current_rotation * 180. / PI
    }
}

pub fn camera_tilt_system(
    mut camera_query: Query<(&mut Transform, &mut CameraTilt), With<Camera>>,
    game_config: ResMut<GameConfig>,
) {
    if let Some(tilt_degree) = game_config.tilt {
        let mut rng = rand::rng();
        let direction = if rng.random_bool(0.5) { 1 } else { -1 };
        let tilt_degree = rng.random_range((tilt_degree / 4)..tilt_degree);
        for (_transform, mut camera_tilt) in &mut camera_query {
            let tilt_radians = (tilt_degree * direction) as f32 * PI / 180.0;
            camera_tilt.target_rotation = tilt_radians;
        }
    }
}

pub fn camera_smooth_rotation(
    mut camera_query: Query<(&mut Transform, &mut CameraTilt), With<Camera>>,
    time: Res<Time>,
) {
    for (mut transform, mut camera_tilt) in &mut camera_query {
        let rotation_diff = camera_tilt.target_rotation - camera_tilt.current_rotation;

        if rotation_diff.abs() < 0.01 {
            camera_tilt.current_rotation = camera_tilt.target_rotation;
        } else {
            let rotation_step = rotation_diff * camera_tilt.rotation_speed * time.delta_seconds();
            camera_tilt.current_rotation += rotation_step;
        }

        transform.rotation = Quat::from_rotation_z(camera_tilt.current_rotation);
    }
}
