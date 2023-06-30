use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::config;

use super::tracer;

#[derive(Component)]
pub(crate) struct Delta;

// #[derive(Resource)]
// pub(crate) struct RunTimer(Timer);

pub(crate) fn spawn(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    bundle: impl Bundle,
) {
    let init_point = Vec3 {
        x: 1.,
        y: 1.,
        z: 0.,
    };

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(config::Delta::SHAPE.into()).into(),
            material: materials.add(ColorMaterial::from(config::Delta::COLOR)),
            transform: Transform {
                scale: config::Delta::SIZE,
                translation: init_point,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert((
            Delta,
            tracer::Tracer {
                last_point: init_point.truncate(),
            },
            bundle,
        ));
}

// pub(crate) fn run_timer(time: Res<Time>, mut run_timer: ResMut<RunTimer>) {
// run_timer.0.tick(time.delta());
// }
//
// pub(crate) fn run(mut delta_query: Query<&mut Transform, With<Delta>>, run_timer: Res<RunTimer>) {
// if !run_timer.0.finished() {
// return;
// }
//
// let mut transform = delta_query.single_mut();
// transform.translation = gumowski_mira::f(transform.translation.truncate()).extend(0.);
// }
