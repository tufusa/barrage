use std::{f32::consts::PI, ops::Sub};

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{
    config,
    in_game::{
        self,
        bullet::{self, bullet_spawn_clock::BulletSpawnClock, collision::BulletCollidable},
        bullets::StraightBullet,
        hp::HP,
    },
};

#[derive(Component)]
pub(crate) struct Normal2;

pub(crate) fn spawn(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    translation: Vec3,
    bundle: impl Bundle,
) {
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(config::enemy::Normal1::SHAPE.into()).into(),
            material: materials.add(ColorMaterial::from(config::enemy::Normal1::COLOR)),
            ..Default::default()
        })
        .insert(SpatialBundle::from_transform(Transform {
            translation,
            scale: config::enemy::Normal1::SIZE,
            ..Default::default()
        }))
        .insert((super::Enemy, HP::new(5), Normal2, BulletCollidable::Enemy))
        .insert(bundle)
        .with_children(|parent| {
            let bullets: Vec<(StraightBullet, f32, u64)> = vec![
                (StraightBullet::new(150.), -PI / 8., 800),
                (StraightBullet::new(150.), 0., 800),
                (StraightBullet::new(150.), PI / 8., 800),
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

pub(crate) fn run(
    mut enemy_query: Query<&mut Transform, With<Normal2>>,
    delta_query: Query<&Transform, (With<in_game::delta::Delta>, Without<Normal2>)>,
    time: Res<Time>,
) {
    let player = delta_query.single();

    enemy_query.iter_mut().for_each(|mut transform| {
        let angle = transform
            .right()
            .truncate()
            .angle_between(player.translation.sub(transform.translation).truncate());
        if transform.translation.y > 250. {
            transform.translation.y -= 50. * time.delta_seconds();
        }
        transform.rotate_z(angle);
    })
}

pub(crate) fn check_despawn(
    mut commands: Commands,
    normal1_query: Query<(&HP, Entity), With<Normal2>>,
) {
    normal1_query.for_each(|(hp, entity)| {
        if hp.hp() > 0 {
            return;
        }

        commands.entity(entity).despawn_recursive();
    })
}
