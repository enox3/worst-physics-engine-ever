use std::f32::consts::FRAC_PI_3;

use rand::Rng;

use crate::{extension::GameVariant, EnabledColliders};
use bevy::{prelude::*, utils::HashSet};
use bevy_ecs_ldtk::{LdtkWorldBundle, LevelSelection};

use crate::{
    audio::AudioEvent, CurrentLevel, CurrentTab, FontHandle, GameConfig, GameKind, GameMode,
    LdtkHandle, Progression, RandomProgressState, ACTIVE_BUTTON, ACTIVE_HOVERED_BUTTON,
    DISABLED_BUTTON, HOVERED_BUTTON, LEVELS, NORMAL_BUTTON, PRESSED_BUTTON,
    REQUIRED_STARS_PER_VARIANT, TEXT_COLOR,
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameMode::Menu), setup)
            .add_systems(OnExit(GameMode::Menu), exit_screen)
            .add_systems(
                Update,
                (rebuild_on_tab_change, button_system).run_if(in_state(GameMode::Menu)),
            );
    }
}

#[derive(Component)]
struct OnMenuScreen;

fn exit_screen(mut commands: Commands, query: Query<Entity, With<OnMenuScreen>>) {
    for entity in &mut query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup(
    mut commands: Commands,
    font: Res<FontHandle>,
    game_kind: Res<State<GameKind>>,
    progression: Res<Progression>,
    asset_server: Res<AssetServer>,
    current_tab: Res<CurrentTab>,
) {
    commands.insert_resource(ClearColor(Color::BLACK));

    // Common style for all buttons on the screen
    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(70.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(5.0)),
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 25.0,
        color: TEXT_COLOR,
        font: font.0.clone(),
    };
    let disabled_button_text_style = TextStyle {
        font_size: 25.0,
        color: TEXT_COLOR * 0.5,
        font: font.0.clone(),
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::rgba(0.3, 0.3, 0.3, 0.5).into(),
                ..default()
            },
            OnMenuScreen,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn(
                TextBundle::from_section(
                    match game_kind.get() {
                        GameKind::Platformer => "Super Duper Platformer",
                        GameKind::Puzzle => "Worst Physics Engine Ever",
                    },
                    TextStyle {
                        font_size: 60.0,
                        color: TEXT_COLOR,
                        font: font.0.clone(),
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(40.0)),
                    ..default()
                })
                .with_text_alignment(TextAlignment::Center),
            );
            parent.spawn(
                TextBundle::from_section(
                    "Let's get those pancakes!",
                    TextStyle {
                        font_size: 25.0,
                        color: TEXT_COLOR,
                        font: font.0.clone(),
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(40.0)),
                    ..default()
                })
                .with_text_alignment(TextAlignment::Center),
            );
            // Tabs
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        flex_wrap: FlexWrap::Wrap,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(8.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|row| {
                    for tab_index in 0..GameVariant::get_number_of_variants() {
                        let required_stars = REQUIRED_STARS_PER_VARIANT * tab_index;
                        let required_stars_left =
                            required_stars.saturating_sub(progression.total_stars());
                        let enabled = progression.total_stars() >= required_stars;
                        let mut button = row.spawn(ButtonBundle {
                            style: Style {
                                width: Val::Px(240.0),
                                height: Val::Px(50.0),
                                margin: UiRect::all(Val::Px(6.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(4.0)),
                                ..default()
                            },
                            background_color: if enabled {
                                NORMAL_BUTTON.into()
                            } else {
                                DISABLED_BUTTON.into()
                            },
                            border_color: BorderColor(HOVERED_BUTTON),
                            ..default()
                        });
                        button.with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                if enabled {
                                    GameVariant::get_name_by_index(tab_index)
                                } else {
                                    format!(
                                        "{} ({})",
                                        GameVariant::get_name_by_index(tab_index),
                                        required_stars_left
                                    )
                                },
                                TextStyle {
                                    font_size: 20.,
                                    ..if enabled {
                                        button_text_style.clone()
                                    } else {
                                        disabled_button_text_style.clone()
                                    }
                                },
                            ));
                        });
                        if enabled {
                            button.insert(ButtonAction::SelectTab(tab_index));
                        }
                    }
                });

            // Top-right: Total star counter with legend icons
            parent
                .spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        right: Val::Px(16.0),
                        top: Val::Px(16.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|row| {
                    row.spawn(TextBundle::from_section(
                        format!("{} ", progression.total_stars()),
                        TextStyle {
                            font_size: 22.0,
                            color: TEXT_COLOR,
                            font: font.0.clone(),
                        },
                    ));
                    for (path,) in [("starGold.png",), ("starSilver.png",), ("starBronze.png",)] {
                        row.spawn(ImageBundle {
                            style: Style {
                                width: Val::Px(22.0),
                                height: Val::Px(22.0),
                                ..default()
                            },
                            image: UiImage::new(asset_server.load(path)),
                            ..default()
                        });
                    }
                });

            // Level
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        max_width: Val::Percent(80.0),
                        flex_wrap: FlexWrap::Wrap,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    let max_levels = if current_tab.0 == 5 {
                        4 // Mirror Variant
                    } else if current_tab.0 == 6 || current_tab.0 == 7 {
                        1 // Random & RandProg variant
                    } else {
                        LEVELS.len()
                    };
                    for i in 0..max_levels {
                        let levels = &progression.tabs[current_tab.0];
                        let enabled = i == 0 || levels[i - 1] != usize::MAX;
                        let mut button = parent.spawn(ButtonBundle {
                            style: button_style.clone(),
                            background_color: if enabled {
                                NORMAL_BUTTON.into()
                            } else {
                                DISABLED_BUTTON.into()
                            },
                            border_color: BorderColor(HOVERED_BUTTON),
                            ..default()
                        });
                        button.with_children(|parent| {
                            let button_text = if current_tab.0 == 6 || current_tab.0 == 7 {
                                "Start".to_string()
                            } else {
                                format!("Level {}", i + 1)
                            };
                            parent.spawn(TextBundle::from_section(
                                button_text,
                                if enabled {
                                    button_text_style.clone()
                                } else {
                                    disabled_button_text_style.clone()
                                },
                            ));
                            if current_tab.0 != 7 && levels[i] < 3 {
                                parent.spawn(ImageBundle {
                                    style: Style {
                                        position_type: PositionType::Absolute,
                                        right: Val::Px(-8.0),
                                        top: Val::Px(-8.0),
                                        width: Val::Px(25.0),
                                        height: Val::Px(25.0),
                                        ..default()
                                    },
                                    image: UiImage::new(asset_server.load(match levels[i] {
                                        0 => "starGold.png",
                                        1 => "starSilver.png",
                                        2 => "starBronze.png",
                                        _ => unreachable!(),
                                    })),
                                    transform: Transform::from_rotation(Quat::from_rotation_z(
                                        -FRAC_PI_3,
                                    )),
                                    ..default()
                                });
                            }
                        });
                        if enabled {
                            button.insert(ButtonAction::Start(i));
                        }
                    }
                });
        });
}

#[derive(Component)]
enum ButtonAction {
    Start(usize),
    SelectTab(usize),
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
fn button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameMode>>,
    world: Res<LdtkHandle>,
    game_kind: Res<State<GameKind>>,
    mut audio_events: EventWriter<AudioEvent>,
    mut current_tab: ResMut<CurrentTab>,
    mut game_config: ResMut<GameConfig>,
    mut random_progress: ResMut<RandomProgressState>,
) {
    for (interaction, mut color, button) in &mut interaction_query {
        *color = match *interaction {
            Interaction::Pressed => {
                audio_events.send(AudioEvent::Click);
                match button {
                    ButtonAction::Start(level) => {
                        match game_kind.get() {
                            GameKind::Platformer => next_state.set(GameMode::Play),
                            GameKind::Puzzle => next_state.set(GameMode::Edit),
                        };

                        let (actual_level, random_config) = if current_tab.0 == 6 {
                            let mut rng = rand::rng();
                            let random_level = rng.random_range(0..LEVELS.len());
                            let random_config = GameConfig::random(&mut rng);
                            (random_level, Some(random_config))
                        } else if current_tab.0 == 7 {
                            let random_level = random_progress.rng.random_range(0..LEVELS.len());
                            let random_config = GameConfig::random(&mut random_progress.rng);
                            random_progress.advance_rng();
                            (random_level, Some(random_config))
                        } else {
                            (*level, None)
                        };

                        let mut coords = HashSet::new();
                        for starter in &LEVELS[actual_level].start_colliders {
                            coords.insert(*starter);
                        }
                        commands.insert_resource(EnabledColliders { coords });
                        commands.insert_resource(LevelSelection::index(actual_level));
                        let progression_level = if current_tab.0 == 6 || current_tab.0 == 7 {
                            0
                        } else {
                            actual_level
                        };
                        commands.insert_resource(CurrentLevel(progression_level));

                        if let Some(config) = random_config {
                            *game_config = config;
                        }

                        commands.spawn(LdtkWorldBundle {
                            ldtk_handle: world.0.clone(),
                            ..Default::default()
                        });
                    }

                    ButtonAction::SelectTab(tab_index) => {
                        current_tab.0 = *tab_index;
                        *game_config = GameVariant::get_config_by_index(current_tab.0);
                    }
                }

                PRESSED_BUTTON.into()
            }
            Interaction::Hovered => match button {
                ButtonAction::SelectTab(tab_index) if current_tab.0 == *tab_index => {
                    ACTIVE_HOVERED_BUTTON.into()
                }
                _ => HOVERED_BUTTON.into(),
            },
            Interaction::None => match button {
                ButtonAction::SelectTab(tab_index) if current_tab.0 == *tab_index => {
                    ACTIVE_BUTTON.into()
                }
                _ => NORMAL_BUTTON.into(),
            },
        }
    }
}

fn rebuild_on_tab_change(
    mut commands: Commands,
    font: Res<FontHandle>,
    game_kind: Res<State<GameKind>>,
    progression: Res<Progression>,
    asset_server: Res<AssetServer>,
    current_tab: Res<CurrentTab>,
    query: Query<Entity, With<OnMenuScreen>>,
) {
    if !current_tab.is_changed() {
        return;
    }
    for entity in &mut query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    setup(
        commands,
        font,
        game_kind,
        progression,
        asset_server,
        current_tab,
    );
}
