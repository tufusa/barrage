use bevy::prelude::*;

pub(crate) fn f(p: Vec2) -> Vec2 {
    const ALPHA: f64 = 0.009;
    const MU: f64 = -0.801;
    const SIGMA: f64 = 0.05;
    const SCALE: f64 = 25.;

    let (x, y) = (p.x as f64 / SCALE, p.y as f64 / SCALE);
    let fx = y + ALPHA * y * (1. - SIGMA * y.powi(2)) + g(x, MU);
    let fy = -x + g(fx, MU);

    return Vec2 {
        x: (fx * SCALE) as f32,
        y: (fy * SCALE) as f32,
    };
}

fn g(x: f64, mu: f64) -> f64 {
    return mu * x + 2. * (1. - mu) * x.powi(2) / (1. + x.powi(2));
}
