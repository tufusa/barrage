use std::f32::consts::PI;

use bevy::{prelude::*, window::*};
use bevy_prototype_lyon::prelude::{Fill, ShapeBundle, Stroke};

use crate::{config, utility::cursor};

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
                transform: Transform {
                    rotation: Quat::from_rotation_z(PI / 2.),
                    scale: config::Delta::SIZE,
                    ..Default::default()
                },
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
                    angle: 0.,
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
    let player_transform = delta_query.single();
    let diff = dir(&input, player_transform) * speed * time.delta_seconds();
    delta_query.single_mut().translation += diff;
}

fn dir(input: &Res<Input<KeyCode>>, transform: &Transform) -> Vec3 {
    let mut dir = Vec3::ZERO;
    if input.pressed(KeyCode::W) || input.pressed(KeyCode::Up) {
        dir += transform.right();
    }
    if input.pressed(KeyCode::A) || input.pressed(KeyCode::Left) {
        dir += transform.up();
    }
    if input.pressed(KeyCode::S) || input.pressed(KeyCode::Down) {
        dir += transform.left();
    }
    if input.pressed(KeyCode::D) || input.pressed(KeyCode::Right) {
        dir += transform.down();
    }
    dir.normalize_or_zero()
}

pub(crate) fn sync_cursor(
    mut delta_query: Query<&mut Transform, With<Delta>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let mut delta = delta_query.single_mut();
    let Some(cursor) = cursor::position(window_query, camera_query) else { return; };
    let angle = delta
        .right()
        .truncate()
        .angle_between(cursor - delta.translation.truncate());
    delta.rotate_z(angle);
}
