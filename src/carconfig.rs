use std::f32::INFINITY;

use glam::Vec3;
use once_cell::sync::Lazy;

const SCALE_FACTOR: f32 = 2.5;
const VERTICAL_OFFSET: f32 = 0.653407;

fn xform(v: Vec3) -> Vec3 {
    Vec3::new(
        v.y,
        v.z - VERTICAL_OFFSET,
        v.x,
    ) / SCALE_FACTOR
}

pub const WHEEL_WIDTH: f32 = (2.501 - 1.58647) / SCALE_FACTOR;
pub const WHEEL_RADIUS: f32 = (2.38572 - 0.653407) / 2.0 / SCALE_FACTOR;
pub const WHEEL_SEGMENTS: usize = 16;

pub const WHEEL_OFFSETS: Lazy<[ Vec3; 4 ]> = Lazy::new(|| {
    [
        xform(Vec3::new(-4.32195, 2.04374, 1.51956)) - HULL_CENTER,
        xform(Vec3::new(-4.32195, -2.04374, 1.51956)) - HULL_CENTER,
        xform(Vec3::new(5.17027, 2.04374, 1.51956)) - HULL_CENTER,
        xform(Vec3::new(5.17027, -2.04374, 1.51956)) - HULL_CENTER,
    ]
});

pub const WHEEL_MASSES: [ f32; 4 ] = [ 9.5, 9.5, 11.5, 11.5 ];

pub const HULL_MASS: f32 = 800.;

// 200mm above ground
pub const HULL_CENTER: Vec3 = Vec3::new(0., 0., 0.2 + VERTICAL_OFFSET / SCALE_FACTOR);

pub const HULL_VERTICES: Lazy<Vec<Vec3>> = Lazy::new(|| {
    let mut vertices = vec![
        Vec3::new(-6.92643, 0.404563, 0.880658),
        Vec3::new(-1.53245, 2.03004, 0.915228),
        Vec3::new(4.12645, 2.03094, 1.08527),
        Vec3::new(5.98529, 1.39484, 1.05198),
        Vec3::new(-6.86986, 0.418811, 1.25187),
        Vec3::new(-4.11091, -0.381144, 2.35673),
        Vec3::new(0.300081, 0., 3.38127),
        Vec3::new(3.16063, 0., 3.4002),
        Vec3::new(6.53832, -1.00915, 2.84627),
    ];

    for mut v in vertices.clone() {
        if v.y != 0. {
            v.y = -v.y;
            vertices.push(v);
        }
    }

    vertices.into_iter().map(xform).collect()
});

pub const HULL_DIMENSIONS: Lazy<Vec3> = Lazy::new(|| {
    let mut min = Vec3::new(INFINITY, INFINITY, INFINITY);
    let mut max = Vec3::new(-INFINITY, -INFINITY, -INFINITY);

    for v in HULL_VERTICES.iter() {
        min = min.min(*v);
        max = max.max(*v);
    }

    max - min
});
