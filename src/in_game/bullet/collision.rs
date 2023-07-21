use bevy::{prelude::*, sprite::collide_aabb::*};

use crate::in_game::{hp::HP, player_bullet::PlayerBullet};

use super::Bullet;

#[derive(Component, PartialEq, Eq, Clone, Copy)]
pub(crate) enum BulletCollidable {
    Player,
    Enemy,
}

pub(crate) fn collision<B: Bullet>(
    mut commands: Commands,
    mut collider_query: Query<(&Transform, &mut HP, &BulletCollidable)>,
    bullet_query: Query<(&Transform, Entity, Option<&PlayerBullet>), With<B>>,
) {
    collider_query.for_each_mut(|(collider_transform, mut hp, &collidable)| {
        bullet_query.for_each(|(bullet_transform, entity, player_bullet)| {
            let collision = collide(
                collider_transform.translation,
                collider_transform.scale.truncate(),
                bullet_transform.translation,
                bullet_transform.scale.truncate(),
            );

            if collision.is_none()
                || collidable == BulletCollidable::Enemy && player_bullet.is_none()
                || collidable == BulletCollidable::Player && player_bullet.is_some()
            {
                return;
            }

            hp.attacked(B::damage());
            B::despawn(&mut commands, entity);
        })
    })
}
