use bevy::prelude::*;

use crate::config::Delta;

impl Delta {
    pub(crate) const SHAPE: shape::RegularPolygon = shape::RegularPolygon {
        radius: 1.,
        sides: 3,
    };
    pub(crate) const SIZE: Vec3 = Vec3 {
        x: 1. * 10.,
        y: 3. * 10.,
        z: 0.,
    };
    pub(crate) const COLOR: Color = Color::rgb(1., 1., 1.);
}
