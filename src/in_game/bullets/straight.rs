use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{
    config,
    in_game::{bullet, bullets},
};

impl bullet::Bullet for bullets::StraightBullet {
    fn run(&self, transform: &mut Transform, time: &Time) {
        transform.translation +=
            self.speed * Vec2::from_angle(self.angle).extend(0.) * time.delta_seconds();
    }
}

pub(crate) fn spawn(
    mut commands: Commands,
    mut new_bullet_event: EventReader<bullet::NewBullet<bullets::StraightBullet>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    new_bullet_event.iter().for_each(|new_bullet| {
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes.add(config::bullets::Straight::SHAPE.into()).into(),
                material: materials.add(ColorMaterial::from(config::bullets::Straight::COLOR)),
                transform: Transform {
                    translation: new_bullet.translation,
                    scale: config::bullets::Straight::SIZE,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(new_bullet.bullet);
    });
}
