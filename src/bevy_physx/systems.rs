use std::ptr::null;
use bevy::prelude::*;
use physx::prelude::*;
use physx::scene::Scene;
use physx::traits::Class;
use physx_sys::{phys_PxCreateDynamic, PxScene_addActor_mut};
use super::{prelude::*, PxRigidDynamic};
use super::{BPxScene, BPxPhysics, BPxRigidDynamic};

#[derive(Component, Deref, DerefMut)]
pub struct BPxInternalRigidDynamic(Owner<PxRigidDynamic>);

pub fn scene_simulate(time: Res<Time>, mut scene: ResMut<BPxScene>) {
    if time.delta_seconds() == 0. { return; }

    scene.simulate(time.delta_seconds(), None, None);
    scene.fetch_results(true).unwrap();
}

pub fn create_dynamic_actors(
    mut commands: Commands,
    mut physics: ResMut<BPxPhysics>,
    mut scene: ResMut<BPxScene>,
    new_actors: Query<(Entity, &BPxRigidDynamic, &GlobalTransform), Without<BPxInternalRigidDynamic>>,
    geometries: Res<Assets<BPxGeometry>>,
    mut materials: ResMut<Assets<BPxMaterial>>,
) {
    for (entity, cfg, transform) in new_actors.iter() {
        let geometry = geometries.get(&cfg.geometry).expect("geometry not found for BPxGeometry");
        let material = materials.get_mut(&cfg.material).expect("material not found for BPxMaterial");

        let mut actor : Owner<PxRigidDynamic> = unsafe {
            RigidDynamic::from_raw(
                phys_PxCreateDynamic(
                    physics.physics_mut().as_mut_ptr(),
                    transform.to_physx().as_ptr(),
                    geometry.as_ptr(),
                    material.as_mut_ptr(),
                    cfg.density,
                    cfg.shape_transform.to_physx().as_ptr(),
                ),
                (),
            )
        }.unwrap();

        unsafe {
            PxScene_addActor_mut(scene.as_mut_ptr(), actor.as_mut_ptr(), null());
        }

        commands.entity(entity)
            .insert(BPxInternalRigidDynamic(actor));
    }
}

pub fn writeback_actors(
    mut actors: Query<(&BPxInternalRigidDynamic, &mut Transform)>
) {
    for (actor, mut transform) in actors.iter_mut() {
        let xform = actor.get_global_pose();
        *transform = xform.to_bevy();
    }
}






/*fn create_drivable_plane(physics: &mut PxPhysics<PxShape>, material: &mut PxMaterial, scene: &mut PxScene) {
    let mut ground_plane = physics
        .create_plane(PxVec3::new(0.0, 1.0, 0.0), 0.0, material, ())
        .unwrap();

    let mut shapes = ground_plane.get_shapes_mut();

    for shape in shapes.iter_mut() {
        let qry_filter_data = unsafe { PxFilterData_new_2(0, 0, 0, DRIVABLE_SURFACE) };
        let sim_filter_data = unsafe { PxFilterData_new_2(COLLISION_FLAG_GROUND, COLLISION_FLAG_GROUND_AGAINST, 0, 0) };

        unsafe {
            PxShape_setQueryFilterData_mut(shape.as_mut_ptr(), &qry_filter_data as *const PxFilterData);
            PxShape_setSimulationFilterData_mut(shape.as_mut_ptr(), &sim_filter_data as *const PxFilterData);
        };
    }

    scene.add_static_actor(ground_plane);
}*/
