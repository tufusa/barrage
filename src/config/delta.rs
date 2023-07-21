use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::config::Delta;

impl Delta {
    pub(crate) const SHAPE: shape::RegularPolygon = shape::RegularPolygon {
        radius: 1.,
        sides: 3,
    };

    pub(crate) const SIZE: Vec3 = Vec3 {
        x: 1. * 5.,
        y: 1. * 5.,
        z: 0.,
    };
    pub(crate) const COLOR: Color = Color::WHITE;

    pub(crate) fn path() -> Path {
        let mut path_builder = PathBuilder::new();
        path_builder.move_to(Vec2::new(f32::cos(PI / 6. * 3.), f32::sin(PI / 6. * 3.)) * 3.);
        path_builder.line_to(Vec2::new(f32::cos(PI / 6. * 7.), f32::sin(PI / 6. * 7.)));
        path_builder.line_to(Vec2::new(f32::cos(PI / 6. * 11.), f32::sin(PI / 6. * 11.)));
        path_builder.close();

        path_builder.build()
    }
}
