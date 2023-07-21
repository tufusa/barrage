use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{Fill, ShapeBundle, Stroke};

use crate::config;

use super::{
    bullet::{
        bullet_source::BulletSource, bullet_spawn_clock::BulletSpawnClock,
        collision::BulletCollidable,
    },
    bullets::PlayerStraightBullet,
    hp::HP,
};

#[derive(Component)]
pub(crate) struct Delta;

pub(crate) fn spawn(commands: &mut Commands, bundle: impl Bundle) {
    commands
        .spawn((
            ShapeBundle {
                path: config::Delta::path(),
                transform: Transform::from_scale(config::Delta::SIZE),
                ..Default::default()
            },
            Stroke::new(config::Delta::COLOR, 0.1),
            Fill::color(Color::NONE),
        ))
        .insert((Delta, HP::new(10), BulletCollidable::Player, bundle))
        .with_children(|parent| {
            parent
                .spawn(SpatialBundle::default())
                .insert(BulletSource {
                    angle: PI / 2.,
                    bullet: PlayerStraightBullet::new(200.),
                })
                .insert(BulletSpawnClock::new(300));
        });
}

pub(crate) fn control(
    input: Res<Input<KeyCode>>,
    mut delta_query: Query<&mut Transform, With<Delta>>,
    time: Res<Time>,
) {
    let speed = 100.;
    let diff = dir(&input).extend(0.) * speed * time.delta_seconds();
    delta_query.single_mut().translation += diff;
}

fn dir(input: &Res<Input<KeyCode>>) -> Vec2 {
    let mut dir = Vec2::ZERO;
    if input.pressed(KeyCode::W) || input.pressed(KeyCode::Up) {
        dir += Vec2::Y;
    }
    if input.pressed(KeyCode::A) || input.pressed(KeyCode::Left) {
        dir += Vec2::NEG_X;
    }
    if input.pressed(KeyCode::S) || input.pressed(KeyCode::Down) {
        dir += Vec2::NEG_Y;
    }
    if input.pressed(KeyCode::D) || input.pressed(KeyCode::Right) {
        dir += Vec2::X;
    }
    dir.normalize_or_zero()
}
