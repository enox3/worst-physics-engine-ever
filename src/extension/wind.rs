use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;
use rand_distr::{Distribution, Normal};

use crate::{GameConfig, OnPlayMode, Player};

const WIND_WIDTH: f32 = 40.;
const WIND_HEIGHT: f32 = 40.;

const LEVEL_HEIGHT: f32 = 320.;
const LEVEL_WIDTH: f32 = 576.;

#[derive(Clone)]
pub struct Wind {
    pub frequency: f32,
    pub speed: f32,
    // Best between 0 and 100 - where 0 is no shift and 100 is a huge shift
    pub strength: f32,
    // Best between 0 and 1 - in 0.1 steps
    pub cooldown: f32,
}
impl Default for Wind {
    fn default() -> Self {
        Self {
            frequency: 4.,
            speed: 250.,
            strength: 50.,
            cooldown: 0.1,
        }
    }
}

#[derive(Resource)]
pub struct WindDirection(pub f32);

#[derive(Resource)]
pub struct WindAtlasHandle(pub Handle<TextureAtlas>);

pub fn setup_wind_spawner(mut commands: Commands, game_config: Res<GameConfig>) {
    if let Some(wind_config) = &game_config.wind {
        commands.spawn(WindSpawner {
            timer: 0.0,
            spawn_interval: wind_config.frequency,
            speed: wind_config.speed,
            strength: wind_config.strength,
        });
    }
}

pub fn setup_wind_atlas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let wind_texture: Handle<Image> = asset_server.load("atlas/wind.png");
    let wind_atlas = TextureAtlas::from_grid(
        wind_texture,
        Vec2::new(WIND_WIDTH, WIND_HEIGHT),
        4,
        1,
        None,
        None,
    );
    let wind_atlas_handle = texture_atlases.add(wind_atlas);
    commands.insert_resource(WindAtlasHandle(wind_atlas_handle));
}

pub fn setup_wind_direction(mut commands: Commands) {
    let mut rng = rand::rng();
    let direction = if rng.random_bool(0.5) { 1.0 } else { -1.0 };
    commands.insert_resource(WindDirection(direction));
}

pub fn wind_spawn_system(
    mut commands: Commands,
    mut spawner_query: Query<&mut WindSpawner>,
    player_transform: Query<&Transform, With<Player>>,
    wind_direction: Res<WindDirection>,
    wind_atlas: Res<WindAtlasHandle>,
    time: Res<Time>,
) {
    // Fixes crash after retry
    if player_transform.is_empty() {
        return;
    }

    for mut spawner in &mut spawner_query {
        spawner.timer += time.delta_seconds();

        if spawner.timer >= spawner.spawn_interval {
            spawner.timer = 0.0;

            let player_y = player_transform.get_single().unwrap().translation.y;

            let mut rng = rand::rng();
            let normal_dist = Normal::new(player_y, LEVEL_HEIGHT / 4.)
                .expect("Failed to create normal distribution");

            let y_pos = normal_dist.sample(&mut rng); // rng.gen_range(0.0..level_height);
            let x_pos = if wind_direction.0 > 0.0 {
                -WIND_WIDTH
            } else {
                LEVEL_WIDTH + 2. * WIND_WIDTH
            };

            commands.spawn((
                SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        index: 0,
                        ..default()
                    },
                    texture_atlas: wind_atlas.0.clone(),
                    transform: Transform::from_xyz(x_pos, y_pos, 10.0),
                    ..default()
                },
                WindEntity {
                    speed: spawner.speed,
                    direction: wind_direction.0,
                    strength: spawner.strength,
                },
                OnPlayMode,
            ));
        }
    }
}

pub fn wind_movement_system(
    mut wind_query: Query<(Entity, &mut Transform, &mut TextureAtlasSprite, &WindEntity)>,
    time: Res<Time>,
) {
    for (_entity, mut transform, mut sprite, wind_entity) in &mut wind_query {
        transform.translation.x += wind_entity.speed * wind_entity.direction * time.delta_seconds();

        sprite.index = ((time.elapsed_seconds() * 8.0).floor() as usize) % 4;

        sprite.flip_x = wind_entity.direction < 0.0;
    }
}

pub fn wind_effect_system(
    mut commands: Commands,
    wind_query: Query<(&Transform, &WindEntity)>,
    mut player_query: Query<
        (Entity, &mut Velocity, &Transform, Option<&mut WindCooldown>),
        With<Player>,
    >,
    game_config: Res<GameConfig>,
) {
    for (player_entity, mut player_velocity, player_transform, wind_cooldown) in &mut player_query {
        if wind_cooldown.is_some() {
            continue;
        }

        for (wind_transform, wind_entity) in &wind_query {
            let distance = wind_transform
                .translation
                .distance(player_transform.translation);

            if distance < 30.0 {
                let wind_impulse =
                    wind_entity.direction * wind_entity.strength * wind_entity.speed * 100.0;
                player_velocity.linvel.x += wind_impulse;
                player_velocity.linvel.x = player_velocity.linvel.x.clamp(-500.0, 500.0);

                commands.entity(player_entity).insert(WindCooldown {
                    timer: 0.0,
                    duration: game_config.wind.clone().unwrap_or_default().cooldown / 10.,
                });
                break;
            }
        }
    }
}

pub fn wind_cleanup_system(
    mut commands: Commands,
    wind_query: Query<(Entity, &Transform, &WindEntity)>,
) {
    for (entity, transform, _wind_entity) in &wind_query {
        if transform.translation.x < -100.0 || transform.translation.x > 900.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn cleanup_wind_cooldowns(
    mut commands: Commands,
    mut cooldown_query: Query<(Entity, &mut WindCooldown)>,
    time: Res<Time>,
) {
    for (entity, mut cooldown) in &mut cooldown_query {
        cooldown.timer += time.delta_seconds();
        if cooldown.timer >= cooldown.duration {
            commands.entity(entity).remove::<WindCooldown>();
        }
    }
}

// Cleanup wind spawners when exiting play mode
pub fn cleanup_wind_spawners(
    mut commands: Commands,
    spawner_query: Query<Entity, With<WindSpawner>>,
) {
    for entity in &spawner_query {
        commands.entity(entity).despawn();
    }
}

#[derive(Component)]
pub struct WindEntity {
    pub speed: f32,
    pub direction: f32,
    pub strength: f32,
}

#[derive(Component)]
pub struct WindSpawner {
    pub timer: f32,
    pub spawn_interval: f32,
    pub speed: f32,
    pub strength: f32,
}

#[derive(Component)]
pub struct WindCooldown {
    pub timer: f32,
    pub duration: f32,
}
