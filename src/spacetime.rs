use bevy::prelude::*;
use chrono::prelude::*;
use nalgebra::{Vector3, UnitQuaternion};

pub const INGAME_TIME_FACTOR: f32 = 1.;

#[derive(Debug, Resource)]
pub struct GameTime {
    // amount of seconds since unix epoch (1970)
    timestamp: i64,
    fraction: f32,
}

impl GameTime {
    pub fn new_today() -> Self {
        let mut date = Utc::now().timestamp();
        date -= date % 86400;
        Self { timestamp: date, fraction: 0. }
    }

    pub fn add_seconds(&mut self, delta: f32) {
        self.fraction += delta;
        let floor = self.fraction.floor();
        self.fraction -= floor;
        self.timestamp += floor as i64;
    }

    /*pub fn as_julian(&self) -> f64 {
        self.timestamp as f64 / 86400.0 + 2440587.5
    }*/
}

#[derive(Debug, Component, Default)]
pub struct GameObject {
    pub position: Vector3<f64>,
    pub rotation: UnitQuaternion<f64>,
}

#[derive(Debug, Resource)]
pub struct GameSpaceOrigin(pub Vector3<f64>);

pub struct SpaceTimePlugin;

impl Plugin for SpaceTimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameTime::new_today());
        app.insert_resource(GameSpaceOrigin(Vector3::new(0., 0., 0.)));
        app.add_system(update_time);
        app.add_system(update_transforms);
    }
}

pub fn update_time(
    time: Res<Time>,
    mut gamedate: ResMut<GameTime>,
) {
    let delta = time.delta_seconds() * INGAME_TIME_FACTOR;
    gamedate.add_seconds(delta);
}

fn update_transforms(
    origin: Res<GameSpaceOrigin>,
    mut query: Query<(&mut Transform, &GameObject)>,
) {
    for (mut transform, object) in query.iter_mut() {
        let pos = object.position - origin.0;
        transform.translation = Vec3::new(pos.x as f32, pos.y as f32, pos.z as f32);

        let rot = object.rotation.as_vector();
        transform.rotation = Quat::from_xyzw(rot.x as f32, rot.y as f32, rot.z as f32, rot.w as f32);
    }
}
