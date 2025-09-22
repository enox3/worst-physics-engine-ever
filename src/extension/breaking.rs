use bevy::prelude::*;

pub fn destroy_colliders_on_timer(
    mut commands: Commands,
    mut colliders: Query<(Entity, &mut DestroyOnPlayerContact)>,
    time: Res<Time>,
) {
    for (entity, mut destroy_timer) in &mut colliders {
        destroy_timer.timer.tick(time.delta());
        if destroy_timer.timer.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[derive(Component)]
pub struct DestroyOnPlayerContact {
    pub timer: Timer,
}

// impl DestroyOnPlayerContact {
//     pub fn new(timer_seconds: f32) -> Self {
//         Self {
//             timer: Timer::from_seconds(timer_seconds, TimerMode::Once),
//         }
//     }
// }
