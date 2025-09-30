mod breaking;
mod current_tab;
mod game_config;
mod game_variant;
mod progression;
mod random_progress;
mod tilting;
mod wind;

pub use breaking::*;
pub use current_tab::*;
pub use game_config::*;
pub use game_variant::*;
pub use progression::*;
pub use random_progress::*;
pub use tilting::*;
pub use wind::*;

const NUM_OF_VARIANTS: usize = 8;
