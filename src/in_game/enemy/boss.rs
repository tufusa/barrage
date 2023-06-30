use std::f32::consts::PI;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{
    config,
    in_game::{bullet, bullets::StraightBullet},
};

#[derive(Component)]
pub(crate) struct Boss;

pub(crate) fn spawn(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(config::enemy::Boss::SHAPE.into()).into(),
            material: materials.add(ColorMaterial::from(config::enemy::Boss::COLOR)),
            ..Default::default()
        })
        .insert(SpatialBundle::from_transform(Transform {
            translation: (100., 100., 1.).into(),
            scale: config::enemy::Boss::SIZE,
            ..Default::default()
        }))
        .insert((super::Enemy, Boss))
        .with_children(|parent| {
            let bullets: Vec<StraightBullet> = vec![
                StraightBullet::new(150., PI / 4. * 1.),
                StraightBullet::new(150., PI / 4. * 2.),
                StraightBullet::new(150., PI / 4. * 3.),
                StraightBullet::new(150., PI / 4. * 4.),
                StraightBullet::new(150., PI / 4. * 5.),
                StraightBullet::new(150., PI / 4. * 6.),
                StraightBullet::new(150., PI / 4. * 7.),
                StraightBullet::new(150., PI / 4. * 8.),
                StraightBullet::new(250., PI / 4. * 1.),
                StraightBullet::new(250., PI / 4. * 2.),
                StraightBullet::new(250., PI / 4. * 3.),
                StraightBullet::new(250., PI / 4. * 4.),
                StraightBullet::new(250., PI / 4. * 5.),
                StraightBullet::new(250., PI / 4. * 6.),
                StraightBullet::new(250., PI / 4. * 7.),
                StraightBullet::new(250., PI / 4. * 8.),
            ];

            bullets.iter().for_each(|bullet| {
                parent
                    .spawn(SpatialBundle::default())
                    .insert(bullet::bullet_source::BulletSource(*bullet));
            });
        });
}

pub(crate) fn run(mut boss_query: Query<&mut Transform, With<Boss>>, time: Res<Time>) {
    let global_angular_velocity1 = 1.3;
    let global_radius1 = 100.;

    let global_angular_velocity2 = 5.7;
    let global_radius2 = 30.;

    let local_angular_velocity = 10.;

    boss_query.iter_mut().for_each(|mut transform| {
        transform.translation.x = f32::cos(time.elapsed_seconds() * global_angular_velocity1)
            * global_radius1
            + f32::sin(time.elapsed_seconds() * global_angular_velocity2) * global_radius2;
        transform.translation.y = f32::sin(time.elapsed_seconds() * global_angular_velocity1)
            * global_radius1
            + f32::cos(time.elapsed_seconds() * global_angular_velocity2) * global_radius2;

        transform.rotate_z(time.delta_seconds() * local_angular_velocity);
    })
}
