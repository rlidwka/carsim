use bevy::prelude::*;
use physx::prelude::*;
use physx::scene::Scene;
use physx::traits::Class;
use physx_sys::phys_PxCreateDynamic;
use super::prelude::*;
use super::{BPxScene, BPxPhysics, BPxRigidDynamic};

#[derive(Component)]
pub struct BPxInternalRigidDynamic;

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

        let actor = unsafe {
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

        scene.add_dynamic_actor(actor);

        commands.entity(entity)
            .insert(BPxInternalRigidDynamic);
    }





/*

        geometry(radius)
        material(st,dy,rest)
        transform
        density
        shape_transform*/


}
