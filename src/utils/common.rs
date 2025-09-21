use bevy::prelude::*;
use bevy_ecs_ldtk::{assets::LdtkProject, GridCoords};

#[derive(Resource, Clone)]
pub struct LevelInfo {
    pub start_colliders: [GridCoords; 2],
    pub thresholds: [usize; 3],
    pub max_colliders: usize,
}

pub const LEVELS: [LevelInfo; 6] = [
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

#[derive(Resource)]
pub struct CurrentLevel(pub usize);

#[derive(Resource)]
pub struct FontHandle(pub Handle<Font>);

#[derive(Resource)]
pub struct LdtkHandle(pub Handle<LdtkProject>);

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum GameKind {
    #[cfg_attr(not(feature = "debug"), default)]
    Platformer,
    #[cfg_attr(feature = "debug", default)]
    Puzzle,
}

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum GameMode {
    #[default]
    Menu,
    Edit,
    Play,
    Won,
    Lost,
    Crash,
}

pub const REQUIRED_STARS_PER_VARIANT: usize = if cfg!(debug_assertions) { 0 } else { 12 };

pub const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const ACTIVE_BUTTON: Color = Color::rgb(0.3, 0.3, 0.3);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const ACTIVE_HOVERED_BUTTON: Color = Color::rgb(0.25, 0.15, 0.15);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.35, 0.35);
pub const DISABLED_BUTTON: Color = Color::rgb(0.1, 0.1, 0.1);
