use std::ops::Sub;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::config;
use crate::in_game::bullet::collision::BulletCollidable;
use crate::in_game::{
    self,
    bullet::{self, new_bullet::NewBullet},
};

use super::HomingBullet;

impl HomingBullet {
    pub(crate) fn new(speed: f32) -> Self {
        return Self { speed, angle: 0. };
    }
}

impl bullet::Bullet for HomingBullet {
    fn damage() -> u32 {
        let damage = 1;
        damage
    }

    fn run(
        mut bullet_query: Query<(&mut Self, &mut Transform)>,
        player_query: Query<
            '_,
            '_,
            &Transform,
            (With<in_game::delta::Delta>, Without<HomingBullet>),
        >,
        time: Res<Time>,
    ) {
        let player = player_query.single();
        bullet_query
            .iter_mut()
            .for_each(|(mut bullet, mut transform)| {
                bullet.angle += Vec2::from_angle(bullet.angle)
                    .angle_between(player.translation.sub(transform.translation).truncate())
                    * 0.2;
                transform.translation +=
                    bullet.speed * Vec2::from_angle(bullet.angle).extend(0.) * time.delta_seconds();
            });
    }

    fn spawn(
        mut commands: Commands,
        mut new_bullet_event: EventReader<NewBullet<HomingBullet>>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        new_bullet_event.iter().for_each(|new_bullet| {
            let mut bullet = new_bullet.bullet;
            bullet.angle = new_bullet.angle;
            commands
                .spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(config::bullets::Straight::SHAPE.into()).into(),
                    material: materials.add(ColorMaterial::from(config::bullets::Straight::COLOR)),
                    transform: Transform {
                        translation: new_bullet.translation.extend(0.),
                        scale: config::bullets::Straight::SIZE,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(bullet)
                .insert(in_game::InGame)
                .insert(BulletCollidable::Enemy);
        });
    }
}
