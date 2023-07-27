use bevy::sprite::collide_aabb::{collide, Collision};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::config;

use crate::in_game::{
    self,
    bullet::{self, new_bullet::NewBullet},
};

use super::ReflectBullet;

impl ReflectBullet {
    pub(crate) fn new(speed: f32, reflection_remain: u32) -> Self {
        return Self {
            speed,
            direction: Vec2::ZERO,
            reflection_remain,
        };
    }

    fn reflect(&mut self, transform: &mut Transform) {
        if self.reflection_remain == 0 {
            return;
        }

        let collision_option = collide(
            transform.translation,
            transform.scale.truncate(),
            Vec3::ZERO,
            config::Window::SIZE,
        );
        let Some(collision) = collision_option else { return; };

        if collision == Collision::Top && self.direction.y > 0.
            || collision == Collision::Bottom && self.direction.y < 0.
        {
            self.direction.y = -self.direction.y;
            self.reflection_remain -= 1;
        } else if collision == Collision::Left && self.direction.x < 0.
            || collision == Collision::Right && self.direction.x > 0.
        {
            self.direction.x = -self.direction.x;
            self.reflection_remain -= 1;
        }
    }

    pub(crate) fn check_despawn(mut commands: Commands, bullet_query: Query<(Entity, &Self)>) {
        bullet_query.for_each(|(entity, bullet)| {
            if bullet.reflection_remain > 0 {
                return;
            }

            commands.entity(entity).despawn();
        })
    }
}

impl bullet::Bullet for ReflectBullet {
    fn damage() -> u32 {
        let damage = 1;
        damage
    }

    fn run(
        mut bullet_query: Query<(&mut Self, &mut Transform)>,
        _player_query: Query<&Transform, (With<in_game::delta::Delta>, Without<ReflectBullet>)>,
        time: Res<Time>,
    ) {
        bullet_query
            .iter_mut()
            .for_each(|(mut bullet, mut transform)| {
                if !bullet.direction.is_normalized() {
                    bullet.direction = bullet.direction.normalize();
                }
                transform.translation +=
                    bullet.speed * bullet.direction.extend(0.) * time.delta_seconds();
                bullet.reflect(&mut transform);
            })
    }

    fn spawn(
        mut commands: Commands,
        mut new_bullet_event: EventReader<NewBullet<ReflectBullet>>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        new_bullet_event.iter().for_each(|new_bullet| {
            let mut bullet = new_bullet.bullet;
            bullet.direction = Vec2::from_angle(new_bullet.angle).normalize();
            commands
                .spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(config::bullets::Reflect::SHAPE.into()).into(),
                    material: materials.add(ColorMaterial::from(config::bullets::Reflect::COLOR)),
                    transform: Transform {
                        translation: new_bullet.translation.extend(0.),
                        scale: config::bullets::Reflect::SIZE,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(bullet)
                .insert(in_game::InGame);
        });
    }
}
