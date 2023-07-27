use bevy::prelude::*;

impl super::Reflect {
    pub(crate) const SHAPE: shape::Circle = shape::Circle {
        radius: 1.,
        vertices: 3,
    };
    pub(crate) const SIZE: Vec3 = Vec3 {
        x: 5.,
        y: 5.,
        z: 0.,
    };
    pub(crate) const COLOR: Color = Color::BLUE;
}
