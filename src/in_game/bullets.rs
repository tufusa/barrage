use bevy::prelude::*;

pub(crate) mod straight;

#[derive(Component, Clone, Copy)]
pub(crate) struct StraightBullet {
    pub(crate) speed: f32,
    pub(crate) angle: f32,
}
