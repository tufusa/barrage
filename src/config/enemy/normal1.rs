use bevy::prelude::*;

impl super::Normal1 {
    pub(crate) const SHAPE: shape::Circle = shape::Circle {
        radius: 1.,
        vertices: 4,
    };
    pub(crate) const SIZE: Vec3 = Vec3 {
        x: 10.,
        y: 10.,
        z: 0.,
    };
    pub(crate) const COLOR: Color = Color::WHITE;
}
