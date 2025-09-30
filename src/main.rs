#[cfg(not(feature = "debug"))]
use bevy::prelude::*;
use worst_physics_engine_ever::{
    AabbPickingBackend, AudioPlugin, CameraTilt, ChestBundle, CrashPlugin, CurrentTab, EditPlugin,
    FontHandle, GameConfig, GameKind, GameMode, LadderBundle, LdtkHandle, LostPlugin, MenuPlugin,
    MobBundle, PlayPlugin, PlayerBundle, Progression, PumpkinsBundle, WallBundle, WonPlugin,
    LEVELS,
};

use bevy_ecs_ldtk::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_rapier2d::prelude::*;

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
            WonPlugin,
            LostPlugin,
            EditPlugin,
            PlayPlugin,
            MenuPlugin,
            CrashPlugin,
            AudioPlugin,
        ))
        .add_systems(Startup, setup)
        .register_ldtk_int_cell::<WallBundle>(1)
        .register_ldtk_int_cell::<LadderBundle>(2)
        .register_ldtk_int_cell::<WallBundle>(3)
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<MobBundle>("Mob")
        .register_ldtk_entity::<ChestBundle>("Chest")
        .register_ldtk_entity::<PumpkinsBundle>("Pumpkins")
        .add_state::<GameMode>()
        .add_state::<GameKind>()
        .run();
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera = Camera2dBundle::default();
    commands.spawn((
        camera,
        CameraTilt {
            target_rotation: 0.0,
            current_rotation: 0.0,
            rotation_speed: 1.0,
        },
    ));

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
        let mut rng = rand::rng();
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
            levels[0..4].to_vec(), // Mirror mode only has 4 levels
            levels.clone(),
            levels.clone(),
        ],
    });
    commands.insert_resource(CurrentTab(0));
    commands.insert_resource(GameConfig::default());
}
