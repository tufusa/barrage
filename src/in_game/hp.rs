use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct HP(u32);

impl HP {
    pub(crate) fn new(hp: u32) -> Self {
        Self(hp)
    }

    pub(crate) fn hp(&self) -> u32 {
        self.0
    }

    pub(crate) fn attacked(&mut self, damage: u32) {
        self.0 = self.0.checked_sub(damage).unwrap_or(0);
    }
}
