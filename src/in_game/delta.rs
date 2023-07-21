use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{Fill, ShapeBundle, Stroke};

use crate::config;

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
        .insert((Delta, bundle));
}

pub(crate) fn control(
    input: Res<Input<KeyCode>>,
    mut delta_query: Query<&mut Transform, With<Delta>>,
    time: Res<Time>,
) {
    let speed = 100.;
    let diff = dir(&input).extend(0.) * speed * time.delta_seconds();
    delta_query.iter_mut().for_each(|mut transform| {
        transform.translation += diff;
    });
}

fn dir(input: &Res<Input<KeyCode>>) -> Vec2 {
    let mut dir = Vec2::ZERO;
    if input.pressed(KeyCode::W) {
        dir += Vec2::Y;
    }
    if input.pressed(KeyCode::A) {
        dir += Vec2::NEG_X;
    }
    if input.pressed(KeyCode::S) {
        dir += Vec2::NEG_Y;
    }
    if input.pressed(KeyCode::D) {
        dir += Vec2::X;
    }
    dir.normalize_or_zero()
}
