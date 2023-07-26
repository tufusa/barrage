use bevy::prelude::*;

use super::Bullet;

#[derive(Event)]
pub(crate) struct NewBullet<B: Bullet> {
    pub(crate) translation: Vec2,
    pub(crate) angle: f32,
    pub(crate) bullet: B,
}
