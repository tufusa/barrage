use bevy::prelude::*;

impl super::Boss {
    pub(crate) const SHAPE: shape::Circle = shape::Circle {
        radius: 1.,
        vertices: 6,
    };
    pub(crate) const SIZE: Vec3 = Vec3 {
        x: 30.,
        y: 30.,
        z: 0.,
    };
    pub(crate) const COLOR: Color = Color::WHITE;
}
