use bevy::ecs::system::Resource;

use crate::extension::NUM_OF_VARIANTS;

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

    fn stars_for_levels(levels: &[usize]) -> usize {
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
