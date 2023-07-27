use bevy::prelude::*;

use crate::in_game::{
    bullet::new_bullet::NewBullet,
    bullets::{HomingBullet, PlayerStraightBullet, ReflectBullet, StraightBullet},
};

impl Plugin for super::NewBulletEvents {
    fn build(&self, app: &mut App) {
        app.add_event::<NewBullet<StraightBullet>>()
            .add_event::<NewBullet<PlayerStraightBullet>>()
            .add_event::<NewBullet<ReflectBullet>>()
            .add_event::<NewBullet<HomingBullet>>();
    }
}
