use bevy::prelude::*;

use super::Bullet;

pub(crate) struct NewBullet<B: Bullet> {
    pub(crate) translation: Vec2,
    pub(crate) bullet: B,
}
