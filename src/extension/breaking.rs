use bevy::prelude::*;

pub fn destroy_colliders_on_timer(
    mut commands: Commands,
    mut colliders: Query<(
        Entity,
        &mut DestroyOnPlayerContact,
        Option<&mut Handle<Image>>,
    )>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    for (entity, mut destroy_timer, maybe_texture) in &mut colliders {
        destroy_timer.timer.tick(time.delta());
        if destroy_timer.timer.finished() {
            if !destroy_timer.broke {
                if let Some(mut texture_handle) = maybe_texture {
                    *texture_handle = asset_server.load("breaking2.png");
                }
                destroy_timer.broke = true;
                destroy_timer.timer = Timer::from_seconds(0.2, TimerMode::Once);
            } else {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

#[derive(Component)]
pub struct DestroyOnPlayerContact {
    pub timer: Timer,
    pub broke: bool,
}

impl DestroyOnPlayerContact {
    pub fn new(timer_seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(timer_seconds, TimerMode::Once),
            broke: false,
        }
    }
}
