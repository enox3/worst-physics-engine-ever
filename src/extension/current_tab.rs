use std::ops::Deref;

use bevy::ecs::system::Resource;

#[derive(Resource, Clone, Copy, Debug, Eq, PartialEq)]
pub struct CurrentTab(pub usize);

impl Deref for CurrentTab {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
