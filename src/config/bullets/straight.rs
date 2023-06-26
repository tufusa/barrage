use bevy::prelude::*;

impl super::Straight {
    pub(crate) const SHAPE: shape::Circle = shape::Circle {
        radius: 1.,
        vertices: 10,
    };
    pub(crate) const SIZE: Vec3 = Vec3 {
        x: 1.,
        y: 1.,
        z: 0.,
    };
    pub(crate) const COLOR: Color = Color::RED;
}
