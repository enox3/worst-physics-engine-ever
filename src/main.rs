use aabb_picking_backend::AabbPickingBackend;
use bevy::prelude::*;
use std::ops::Deref;

use bevy_ecs_ldtk::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_rapier2d::prelude::*;

mod aabb_picking_backend;
mod audio;
mod components;
mod crash;
mod edit;
mod lost;
mod menu;
mod play;
mod won;

fn main() {
    App::new()
        .add_plugins((
            EmbeddedAssetPlugin {
                mode: bevy_embedded_assets::PluginMode::ReplaceDefault,
            },
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Worst Physics Engine Ever".to_string(),
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                }),
            LdtkPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            DefaultPickingPlugins,
            AabbPickingBackend,
        ))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, 0.0),
            ..Default::default()
        })
        .insert_resource(LdtkSettings {
            ..Default::default()
        })
        .add_plugins((
            won::WonPlugin,
            lost::LostPlugin,
            edit::EditPlugin,
            play::PlayPlugin,
            menu::MenuPlugin,
            crash::CrashPlugin,
            audio::AudioPlugin,
        ))
        .add_systems(Startup, setup)
        .register_ldtk_int_cell::<components::WallBundle>(1)
        .register_ldtk_int_cell::<components::LadderBundle>(2)
        .register_ldtk_int_cell::<components::WallBundle>(3)
        .register_ldtk_entity::<components::PlayerBundle>("Player")
        .register_ldtk_entity::<components::MobBundle>("Mob")
        .register_ldtk_entity::<components::ChestBundle>("Chest")
        .register_ldtk_entity::<components::PumpkinsBundle>("Pumpkins")
        .add_state::<GameMode>()
        .add_state::<GameKind>()
        .run();
}

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum GameKind {
    #[cfg_attr(not(feature = "debug"), default)]
    Platformer,
    #[cfg_attr(feature = "debug", default)]
    Puzzle,
}

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum GameMode {
    #[default]
    Menu,
    Edit,
    Play,
    Won,
    Lost,
    Crash,
}

const REQUIRED_STARS_PER_VARIANT: usize = if cfg!(debug_assertions) { 0 } else { 12 };

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const ACTIVE_BUTTON: Color = Color::rgb(0.3, 0.3, 0.3);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const ACTIVE_HOVERED_BUTTON: Color = Color::rgb(0.25, 0.15, 0.15);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.35, 0.35);
const DISABLED_BUTTON: Color = Color::rgb(0.1, 0.1, 0.1);

#[derive(Resource)]
pub struct FontHandle(Handle<Font>);

#[derive(Resource)]
pub struct LdtkHandle(Handle<LdtkProject>);

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera = Camera2dBundle::default();
    commands.spawn(camera);

    let world = asset_server.load("Typical_2D_platformer_example.ldtk");
    commands.insert_resource(LdtkHandle(world));

    let font = asset_server.load("PublicPixel-z84yD.ttf");
    commands.insert_resource(FontHandle(font));

    #[cfg(not(feature = "debug"))]
    let levels = vec![usize::MAX; LEVELS.len()];
    #[cfg(feature = "debug")]
    let levels = {
        use rand::Rng;
        let mut levels = vec![];
        let mut rng = rand::thread_rng();
        for _ in 0..LEVELS.len() {
            levels.push(rng.gen_range(0..3));
        }
        levels
    };

    commands.insert_resource(Progression {
        tabs: [
            levels.clone(),
            levels.clone(),
            levels.clone(),
            levels.clone(),
            levels.clone(),
            levels.clone(),
            levels.clone(),
            levels.clone(),
        ],
    });
    commands.insert_resource(CurrentTab(0));
    commands.insert_resource(GameConfig::default());
}

#[derive(Resource, Clone)]
pub struct LevelInfo {
    pub start_colliders: [GridCoords; 2],
    pub thresholds: [usize; 3],
    pub max_colliders: usize,
}

const LEVELS: [LevelInfo; 6] = [
    LevelInfo {
        start_colliders: [GridCoords { x: 5, y: 5 }, GridCoords { x: 30, y: 5 }],
        thresholds: [5, 8, 10],
        max_colliders: 20,
    },
    LevelInfo {
        start_colliders: [GridCoords { x: 5, y: 5 }, GridCoords { x: 30, y: 5 }],
        thresholds: [5, 8, 10],
        max_colliders: 20,
    },
    LevelInfo {
        start_colliders: [GridCoords { x: 5, y: 5 }, GridCoords { x: 30, y: 5 }],
        thresholds: [5, 8, 10],
        max_colliders: 20,
    },
    LevelInfo {
        start_colliders: [GridCoords { x: 5, y: 5 }, GridCoords { x: 30, y: 5 }],
        thresholds: [5, 8, 10],
        max_colliders: 20,
    },
    LevelInfo {
        start_colliders: [GridCoords { x: 1, y: 15 }, GridCoords { x: 34, y: 1 }],
        thresholds: [7, 10, 13],
        max_colliders: 20,
    },
    LevelInfo {
        start_colliders: [GridCoords { x: 1, y: 15 }, GridCoords { x: 34, y: 1 }],
        thresholds: [7, 10, 13],
        max_colliders: 20,
    },
];

const NUM_OF_VARIANTS: usize = 8;

#[derive(Resource)]
struct CurrentLevel(usize);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GameVariant {
    Normal,
    Iced,
    BreakingColliders,
    Mirrored,
    Windy,
    TiltMode,
    RandomProgression,
    Random,
}

impl GameVariant {
    pub fn get_config(&self) -> GameConfig {
        match self {
            GameVariant::Normal => GameConfig { movement_speed: 1. },
            GameVariant::Iced => GameConfig { movement_speed: 2. },
            GameVariant::BreakingColliders => GameConfig { movement_speed: 3. },
            GameVariant::Mirrored => GameConfig { movement_speed: 4. },
            GameVariant::Windy => GameConfig { movement_speed: 5. },
            GameVariant::TiltMode => GameConfig { movement_speed: 1. },
            GameVariant::RandomProgression => GameConfig { movement_speed: 1. },
            GameVariant::Random => GameConfig { movement_speed: 1. },
        }
    }
    pub fn get_config_by_index(index: usize) -> GameConfig {
        Self::get_variant_by_index(index).get_config()
    }
    pub fn get_variant_by_index(index: usize) -> Self {
        match index {
            0 => GameVariant::Normal,
            1 => GameVariant::Iced,
            2 => GameVariant::BreakingColliders,
            3 => GameVariant::Mirrored,
            4 => GameVariant::Windy,
            5 => GameVariant::TiltMode,
            6 => GameVariant::RandomProgression,
            7 => GameVariant::Random,
            _ => GameVariant::Normal,
        }
    }
    pub fn get_name_by_index(index: usize) -> String {
        match index {
            0 => "Normal",
            1 => "Iced",
            2 => "Break",
            3 => "Mirror",
            4 => "Wind",
            5 => "Tilt",
            6 => "RandProg",
            7 => "Rand",
            _ => "Normal",
        }
        .into()
    }
    pub fn get_number_of_variants() -> usize {
        // std::mem::variant_count::<Self>()
        NUM_OF_VARIANTS
    }
}

#[derive(Resource, Clone)]
pub struct Progression {
    pub tabs: [Vec<usize>; NUM_OF_VARIANTS],
}
impl Progression {
    pub fn total_stars(&self) -> usize {
        self.tabs
            .iter()
            .map(|levels| Progression::stars_for_levels(levels))
            .sum()
    }

    pub fn total_stars_for_tab(&self, tab_index: usize) -> usize {
        Progression::stars_for_levels(&self.tabs[tab_index])
    }

    fn stars_for_levels(levels: &Vec<usize>) -> usize {
        levels
            .iter()
            .filter_map(|v| match *v {
                0 => Some(3usize),
                1 => Some(2usize),
                2 => Some(1usize),
                _ => None,
            })
            .sum()
    }
}

#[derive(Resource)]
pub struct GameConfig {
    pub movement_speed: f32,
}
impl Default for GameConfig {
    fn default() -> Self {
        GameConfig { movement_speed: 1. }
    }
}

#[derive(Resource, Clone, Copy, Debug, Eq, PartialEq)]
pub struct CurrentTab(pub usize);

impl Deref for CurrentTab {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
