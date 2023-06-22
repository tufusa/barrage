use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::config;

#[derive(Component)]
pub(crate) struct Delta;

pub(crate) fn spawn(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    bundle: impl Bundle,
) {
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(config::Delta::SHAPE.into()).into(),
            material: materials.add(ColorMaterial::from(config::Delta::COLOR)),
            transform: Transform {
                scale: config::Delta::SIZE,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert((Delta, bundle));
}

pub(crate) fn run(mut delta_query: Query<&mut Transform, With<Delta>>, time: Res<Time>) {
    let mut transform = delta_query.single_mut();
    let t = time.elapsed().as_secs_f32();

    transform.translation.x = 200. * f32::cos(t);
    transform.translation.y = 200. * f32::sin(t);
}
