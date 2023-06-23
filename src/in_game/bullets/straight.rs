use bevy::prelude::*;

use crate::in_game::{bullet::Bullet, bullets};

impl Bullet for bullets::StraightBullet {
    fn run(&self, transform: &mut Transform) {
        transform.translation += self.speed * Vec2::from_angle(self.angle).extend(0.)
    }
}
