use std::f32::consts::PI;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{
    config,
    in_game::{
        bullet::{self, bullet_spawn_clock::BulletSpawnClock, collision::BulletCollidable},
        bullets::StraightBullet,
        hp::HP,
    },
};

#[derive(Component)]
pub(crate) struct Boss;

pub(crate) fn spawn(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    bundle: impl Bundle,
) {
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(config::enemy::Boss::SHAPE.into()).into(),
            material: materials.add(ColorMaterial::from(config::enemy::Boss::COLOR)),
            ..Default::default()
        })
        .insert(SpatialBundle::from_transform(Transform {
            translation: (0., 300., 1.).into(),
            scale: config::enemy::Boss::SIZE,
            ..Default::default()
        }))
        .insert((super::Enemy, HP::new(200), Boss, BulletCollidable::Enemy))
        .insert(bundle)
        .with_children(|parent| {
            let bullets: Vec<(StraightBullet, f32, u64)> = vec![
                (StraightBullet::new(150.), PI / 4. * 1., 100),
                (StraightBullet::new(150.), PI / 4. * 2., 100),
                (StraightBullet::new(150.), PI / 4. * 3., 100),
                (StraightBullet::new(150.), PI / 4. * 4., 100),
                (StraightBullet::new(150.), PI / 4. * 5., 100),
                (StraightBullet::new(150.), PI / 4. * 6., 100),
                (StraightBullet::new(150.), PI / 4. * 7., 100),
                (StraightBullet::new(150.), PI / 4. * 8., 100),
                (StraightBullet::new(250.), PI / 4. * 1., 200),
                (StraightBullet::new(250.), PI / 4. * 2., 200),
                (StraightBullet::new(250.), PI / 4. * 3., 200),
                (StraightBullet::new(250.), PI / 4. * 4., 200),
                (StraightBullet::new(250.), PI / 4. * 5., 200),
                (StraightBullet::new(250.), PI / 4. * 6., 200),
                (StraightBullet::new(250.), PI / 4. * 7., 200),
                (StraightBullet::new(250.), PI / 4. * 8., 200),
            ];

            bullets.iter().for_each(|(bullet, angle, millis)| {
                parent
                    .spawn(SpatialBundle::default())
                    .insert(bullet::bullet_source::BulletSource {
                        bullet: *bullet,
                        angle: *angle,
                    })
                    .insert(BulletSpawnClock::new(*millis));
            });
        });
}

pub(crate) fn run(mut boss_query: Query<&mut Transform, With<Boss>>, time: Res<Time>) {
    let global_angular_velocity1 = 1.3;
    let global_radius1 = 100.;

    let local_angular_velocity = 1.5;

    boss_query.iter_mut().for_each(|mut transform| {
        transform.translation.x =
            f32::cos(time.elapsed_seconds() * global_angular_velocity1) * global_radius1;

        transform.translation.y =
            f32::sin(time.elapsed_seconds() * global_angular_velocity1) * global_radius1;

        transform.rotate_z(time.delta_seconds() * local_angular_velocity);
    })
}

pub(crate) fn check_despawn(mut commands: Commands, boss_query: Query<(&HP, Entity), With<Boss>>) {
    boss_query.for_each(|(hp, entity)| {
        if hp.hp() > 0 {
            return;
        }

        commands.entity(entity).despawn_recursive();
    })
}
