use bevy::prelude::*;

use crate::in_game::{enemy, InGame};

#[derive(Component)]
pub(crate) struct Phase1;

pub(crate) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let enemy1s: Vec<Vec3> = vec![
        Vec3::new(-150., 350., 0.),
        Vec3::new(-60., 350., 0.),
        Vec3::new(60., 350., 0.),
        Vec3::new(150., 350., 0.),
    ];

    enemy1s.iter().for_each(|translation| {
        enemy::normal1::spawn(
            &mut commands,
            &mut meshes,
            &mut materials,
            *translation,
            (InGame, Phase1),
        );
    });
}

pub(crate) fn cleanup(mut commands: Commands, phase1_query: Query<Entity, With<Phase1>>) {
    phase1_query.for_each(|entity| {
        commands.entity(entity).despawn();
    })
}
