use bevy::prelude::*;
use bevy::input::mouse::{MouseWheel, MouseMotion};
use std::f32::consts::{PI, FRAC_PI_2};
use nalgebra::{Vector3, UnitQuaternion};
use crate::spacetime::{GameObject, GameSpaceOrigin, INGAME_TIME_FACTOR};

const G_CONSTANT_MPS: f64 = 9.8;

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameSpaceOrigin(Vector3::new(0., 0., 0.)));
        app.add_startup_system(create_player);
        app.add_system(update_camera);
        app.add_system(apply_controls);
        app.add_system(car_physics.after(apply_controls));
        app.add_system(apply_camera_controls.after(update_camera));
    }
}

#[derive(Component)]
struct PlayerControlled;

#[derive(Component, Debug)]
struct PlayerCamera {
    gimbal_x_rad: f32,
    gimbal_y_rad: f32,
    distance: f32,
    last_rotation: Quat,
}

#[derive(Debug, Component, Default)]
pub struct GameCar {
    // TODO: lateral speed
    pub speed: f64,//Vector3<f64>,
    pub accel_g: f64,
    pub steering: f64,
}

impl Default for PlayerCamera {
    fn default() -> Self {
        Self {
            gimbal_x_rad: 0.,
            gimbal_y_rad: 20f32.to_radians(),
            distance: 3.,
            last_rotation: default()
        }
    }
}

fn create_player(mut commands: Commands, assets: Res<AssetServer>) {
    let rotation = UnitQuaternion::from_euler_angles(0.0, 0.0, std::f64::consts::FRAC_PI_2);

    commands
        .spawn(SpatialBundle::default())
        .insert(PlayerControlled)
        .insert(GameObject {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation,
        })
        .insert(GameCar::default())
        .insert(Name::new("player_car"))
        .with_children(|parent| {
            parent.spawn(SceneBundle {
                scene: assets.load("dodge_charger.glb#Scene0"),
                transform: Transform {
                    rotation: Quat::from_euler(EulerRot::XYZ, PI, 0.0, -FRAC_PI_2),
                    scale: Vec3::new(0.1, 0.1, 0.1),
                    translation: Vec3::new(-0.04, 0.0, 0.0),
                },
                ..default()
            });
        });

    // physical model uses nalgebra's UnitQuaternion<f64>, bevy uses glam's Quat (always f32)
    fn nalgebra_quat_to_glam(q: UnitQuaternion<f64>) -> Quat {
        let vec = q.as_vector();
        Quat::from_xyzw(vec.x as f32, vec.y as f32, vec.z as f32, vec.w as f32)
    }

    commands
        .spawn(Camera3dBundle::default())
        .insert(PlayerCamera {
            last_rotation: nalgebra_quat_to_glam(rotation),
            ..default()
        })
        .insert(Name::new("orbit_camera"));
}


// TODO: rewrite this using real physics
// https://asawicki.info/Mirror/Car%20Physics%20for%20Games/Car%20Physics%20for%20Games.html
pub fn car_physics(
    time: Res<Time>,
    mut query: Query<(&mut GameObject, &mut GameCar)>,
) {
    let delta = time.delta_seconds() * INGAME_TIME_FACTOR;

    for (mut object, mut car) in query.iter_mut() {
        let object = &mut *object;
        //object.position += car.speed;
        object.position += object.rotation * Vector3::new(0., 0., -1.) * car.speed;

        //let speed_vector : Vector3<f64> = Vector3::new(0., 0., -1.) * car.accel_g * G_CONSTANT_MPS;

        const MAX_SPEED : f64 = 0.15; // TODO: mps
        let drag = (car.speed * 0.5).max(0.005) * delta as f64; // TODO: real physics
        car.speed = (car.speed + car.accel_g * G_CONSTANT_MPS * delta as f64 - drag).clamp(0.0, MAX_SPEED);

        let steer_delta = time.delta_seconds() * INGAME_TIME_FACTOR * car.steering as f32 * (car.speed as f32 * 20.0).clamp(0.0, 1.0);
        object.rotation = UnitQuaternion::from_axis_angle(&(object.rotation * Vector3::x_axis()), steer_delta as f64) * object.rotation;
    }
}


fn apply_controls(
    mut player_query: Query<&mut GameCar, With<PlayerControlled>>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
) {
    // TODO: this should be an acceleration curve
    const MAX_ACCEL_G : f64 = 0.5;
    const MAX_DECEL_G : f64 = 1.;
    const ACCEL_FACTOR : f32 = 0.005;
    const DECEL_FACTOR : f32 = 0.02;
    const MAX_STEERING : f64 = 1.;

    let mut object = player_query.single_mut();

    if keys.pressed(KeyCode::W) {
        let delta = time.delta_seconds() * INGAME_TIME_FACTOR * ACCEL_FACTOR;
        object.accel_g = (object.accel_g + delta as f64).clamp(-MAX_DECEL_G, MAX_ACCEL_G);
    } else if keys.pressed(KeyCode::S) {
        let delta = time.delta_seconds() * INGAME_TIME_FACTOR * DECEL_FACTOR;
        object.accel_g = (object.accel_g - delta as f64).clamp(-MAX_DECEL_G, MAX_ACCEL_G);
    } else {
        object.accel_g = 0.0;
    }

    object.steering = 0.0;
    if keys.pressed(KeyCode::A) {
        object.steering += MAX_STEERING;
    }
    if keys.pressed(KeyCode::D) {
        object.steering -= MAX_STEERING;
    }
    /*if keys.pressed(KeyCode::A) {
        let delta = time.delta_seconds() * INGAME_TIME_FACTOR * DEG_PER_SEC.to_radians();
        object.rotation = UnitQuaternion::from_axis_angle(&(object.rotation * Vector3::x_axis()), -delta as f64) * object.rotation;
    }
    if keys.pressed(KeyCode::D) {
        let delta = time.delta_seconds() * INGAME_TIME_FACTOR * DEG_PER_SEC.to_radians();
        object.rotation = UnitQuaternion::from_axis_angle(&(object.rotation * Vector3::x_axis()), delta as f64) * object.rotation;
    }*/
}


fn apply_camera_controls(
    mut scroll_events: EventReader<MouseWheel>,
    mut move_events: EventReader<MouseMotion>,
    buttons: Res<Input<MouseButton>>,
    mut camera_query: Query<&mut PlayerCamera>,
) {
    let mut camera = camera_query.single_mut();

    for ev in scroll_events.iter() {
        camera.distance = (camera.distance * 1.1f32.powf(-ev.y)).clamp(1., 20.);
    }

    const MOUSE_SENSITIVITY: f32 = 0.003;
    if buttons.pressed(MouseButton::Middle) {
        for ev in move_events.iter() {
            camera.gimbal_x_rad -= ev.delta.x * MOUSE_SENSITIVITY;
            camera.gimbal_y_rad = (camera.gimbal_y_rad + ev.delta.y * MOUSE_SENSITIVITY).clamp(-PI/2.2, PI/2.2);
        }
    }
}

fn update_camera(
    origin: Res<GameSpaceOrigin>,
    player_query: Query<&GameObject, (With<PlayerControlled>, Without<PlayerCamera>)>,
    mut camera_query: Query<(&mut Transform, &mut PlayerCamera)>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();
    let object = player_query.single();
    let (mut transform, mut camera) = camera_query.single_mut();

    let v = object.rotation.as_vector();
    let r = Quat::from_xyzw(v.x as f32, v.y as f32, v.z as f32, v.w as f32);
    camera.last_rotation = r.slerp(camera.last_rotation, 1. - delta * 10.);

    let quat = Quat::from_euler(EulerRot::XYZ, camera.gimbal_x_rad, camera.gimbal_y_rad, 0.);

    let pos = object.position - origin.0;
    transform.translation = Vec3::new(pos.x as f32, pos.y as f32, pos.z as f32) +
        (camera.last_rotation * quat * Vec3::Z) * camera.distance;

    transform.look_at(Vec3::new(pos.x as f32, pos.y as f32, pos.z as f32), camera.last_rotation * Vec3::X);
}
