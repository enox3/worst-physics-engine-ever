use crate::{extension::NUM_OF_VARIANTS, GameConfig, SlideFactors};

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
            GameVariant::Normal => GameConfig::default(),
            GameVariant::Iced => GameConfig {
                slide: Some(SlideFactors {
                    acceleration: 0.1,
                    deceleration: 0.98,
                }),
                ..GameConfig::default()
            },
            GameVariant::BreakingColliders => GameConfig {
                movement_speed: 3.,
                ..GameConfig::default()
            },
            GameVariant::Mirrored => GameConfig {
                movement_speed: 4.,
                ..GameConfig::default()
            },
            GameVariant::Windy => GameConfig {
                movement_speed: 5.,
                ..GameConfig::default()
            },
            GameVariant::TiltMode => GameConfig {
                movement_speed: 1.,
                ..GameConfig::default()
            },
            GameVariant::RandomProgression => GameConfig {
                movement_speed: 1.,
                ..GameConfig::default()
            },
            GameVariant::Random => GameConfig {
                movement_speed: 1.,
                ..GameConfig::default()
            },
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
