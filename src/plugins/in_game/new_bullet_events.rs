use bevy::prelude::*;

use crate::in_game::{bullet::new_bullet::NewBullet, bullets::StraightBullet};

impl Plugin for super::NewBulletEvents {
    fn build(&self, app: &mut App) {
        app.add_event::<NewBullet<StraightBullet>>();
    }
}
