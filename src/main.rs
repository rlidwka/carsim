#![warn(clippy::manual_assert)]
#![warn(clippy::semicolon_if_nothing_returned)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use bevy::prelude::*;
mod bevy_physx;
use bevy_physx::*;
mod flying_camera;
use flying_camera::*;
use physx::{prelude::*, traits::Class};

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
    .insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0 / 5.0f32,
    })
    .insert_resource(Msaa::default())
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            present_mode: bevy::window::PresentMode::Immediate,
            ..default()
        },
        ..default()
    }))
    .add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin)
    .add_system(bevy::window::close_on_esc)
    .add_plugin(BPxPlugin {
        vehicles: true,
        cooking: true,
        debugger: true,
        ..default()
    })
    .add_plugin(FlyingCameraPlugin)
    .add_startup_system(spawn_light)
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_plane)
    .add_startup_system(spawn_stack)
    .add_startup_system(spawn_dynamic)
    .run();
}

fn spawn_light(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(FlyingCameraBundle {
        flying_camera: FlyingCamera {
            distance: 60.,
            ..default()
        },
        ..default()
    });

}

fn spawn_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
}

fn spawn_stack(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut physics: ResMut<BPxPhysics>,
    mut px_geometries: ResMut<Assets<BPxGeometry>>,
    mut px_materials: ResMut<Assets<BPxMaterial>>,
) {
    const WIDTH: f32 = 0.5;
    const SIZE: usize = 10;

    let mesh = meshes.add(Mesh::from(shape::Cube { size: WIDTH }));
    let material = materials.add(Color::rgb(0.8, 0.7, 0.6).into());

    let px_geometry = px_geometries.add(PxBoxGeometry::new(WIDTH / 2., WIDTH / 2., WIDTH / 2.).into());
    let px_material = px_materials.add(physics.create_material(0.5, 0.5, 0.6, ()).unwrap().into());

    for i in 0..SIZE {
        for j in 0..SIZE-i {
            let transform = Transform::from_translation(Vec3::new(
                ((j * 2) as f32 - (SIZE - i) as f32) / 2. * WIDTH,
                (i * 2 + 1) as f32 / 2. * WIDTH + 10.,
                0.,
            ));

            commands.spawn_empty()
                .insert(PbrBundle {
                    mesh: mesh.clone(),
                    material: material.clone(),
                    transform,
                    ..default()
                })
                .insert(BPxRigidDynamic {
                    material: px_material.clone(),
                    geometry: px_geometry.clone(),
                    density: 10.,
                    ..default()
                });
        }
    }
/*


	PxShape* shape = gPhysics->createShape(PxBoxGeometry(halfExtent, halfExtent, halfExtent), *gMaterial);
	for(PxU32 i=0; i<size;i++)
	{
		for(PxU32 j=0;j<size-i;j++)
		{
			PxTransform localTm(PxVec3(PxReal(j*2) - PxReal(size-i), PxReal(i*2+1), 0) * halfExtent);
			PxRigidDynamic* body = gPhysics->createRigidDynamic(t.transform(localTm));
			body->attachShape(*shape);
			PxRigidBodyExt::updateMassAndInertia(*body, 10.0f);
			gScene->addActor(*body);
		}
	}
	shape->release();



    commands.spawn_empty()
    .insert(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, -1.5, 0.0),
        ..default()
    })
    .insert(bevy_physx::BPxRigidDynamic {
        ..default()
    })
    .insert(bevy_rapier3d::prelude::RigidBody::Dynamic);

commands.spawn_empty()
    .insert(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    })
    .insert(bevy_physx::BPxRigidDynamic {
        ..default()
    })
    .insert(bevy_rapier3d::prelude::RigidBody::Dynamic);
*/
}

fn spawn_dynamic(mut commands: Commands) {
}
