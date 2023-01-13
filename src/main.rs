use std::{ffi::c_void, f32::consts::PI, ptr::{null, null_mut}, mem::MaybeUninit};

use physx::{prelude::*, physics::{PhysicsFoundationBuilder, PxPhysics}, cooking::{self, ConvexMeshCookingResult, PxConvexMeshDesc, PxCooking, TriangleMeshCookingResult, PxTriangleMeshDesc}, convex_mesh::ConvexMesh, triangle_mesh::TriangleMesh, traits::Class};
use physx_sys::{PxConvexFlag, PxConvexFlags, PxRigidActorExt_createExclusiveShape_mut_1, PxConvexMeshGeometryFlags, PxConvexMeshGeometryFlag, PxShapeFlags, PxShapeFlag, PxRigidActor, PxMeshScale, PxMeshScale_new, PxRigidBodyExt_updateMassAndInertia_mut_1, PxRigidBodyExt_updateMassAndInertia_mut, PxRigidBody, phys_PxInitVehicleSDK, PxSerialization_createSerializationRegistry_mut, phys_PxCloseVehicleSDK, phys_PxVehicleSetBasisVectors, phys_PxVehicleSetUpdateMode, PxVehicleUpdateMode, PxVehicleDrive4W, PxVehicleDrive4W_allocate_mut, PxVehicleDrive4W_setup_mut, PxVehicleWheelData_new, PxVehicleWheelsSimData_allocate_mut, PxVehicleWheelsSimData_setWheelCentreOffset_mut, PxVehicleNoDrive_allocate_mut, PxVehicleNoDrive_setup_mut, PxVehicleWheelsSimData_setWheelData_mut, phys_PxCreateDynamic_1, phys_PxCreateDynamic, PxPhysics_createRigidDynamic_mut, PxVehicleNoDrive_setDriveTorque_mut, phys_PxVehicleUpdates, PxVehicleNoDrive, PxVehicleWheels, PxMeshGeometryFlag, PxMeshGeometryFlags, PxVehicleTireData_new, PxVehicleWheelsSimData_setTireData_mut, PxVehicleWheelsSimData_setSuspTravelDirection_mut, PxVehicleSuspensionData_new, PxVehicleWheelsSimData_setSuspensionData_mut, PxVehicleWheelsSimData_setSuspForceAppPointOffset_mut, PxVehicleWheelsSimData_setTireForceAppPointOffset_mut, PxVehicleWheelsSimData_setSceneQueryFilterData_mut, PxVehicleWheelsSimData_setWheelShapeMapping_mut, phys_PxVehicleComputeSprungMasses, phys_PxVehicleSuspensionRaycasts, PxScene_createBatchQuery_mut, PxBatchQueryDesc_new, PxRaycastQueryResult, PxRaycastHit, PxFilterData, PxQueryHitType, PxFilterData_new_1, PxFilterData_new_2, PxShape_setQueryFilterData_mut, PxShape_setSimulationFilterData_mut, PxHitFlags, PxVehicleDrivableSurfaceToTireFrictionPairs, PxVehicleDrivableSurfaceToTireFrictionPairs_allocate_mut, PxVehicleDrivableSurfaceToTireFrictionPairs_setup_mut, PxVehicleDrivableSurfaceToTireFrictionPairs_setTypePairFriction_mut, PxVehicleDrivableSurfaceType};

type PxMaterial = physx::material::PxMaterial<()>;
type PxShape = physx::shape::PxShape<(), PxMaterial>;
type PxArticulationLink = physx::articulation_link::PxArticulationLink<(), PxShape>;
type PxRigidStatic = physx::rigid_static::PxRigidStatic<(), PxShape>;
type PxRigidDynamic = physx::rigid_dynamic::PxRigidDynamic<(), PxShape>;
type PxArticulation = physx::articulation::PxArticulation<(), PxArticulationLink>;
type PxArticulationReducedCoordinate =
    physx::articulation_reduced_coordinate::PxArticulationReducedCoordinate<(), PxArticulationLink>;
type PxScene = physx::scene::PxScene<
    (),
    PxArticulationLink,
    PxRigidStatic,
    PxRigidDynamic,
    PxArticulation,
    PxArticulationReducedCoordinate,
    OnCollision,
    OnTrigger,
    OnConstraintBreak,
    OnWakeSleep,
    OnAdvance,
>;

mod carconfig;
use carconfig::*;

mod utils;

/// Next up, the simulation event callbacks need to be defined, and possibly an
/// allocator callback as well.
struct OnCollision;
impl CollisionCallback for OnCollision {
    fn on_collision(
        &mut self,
        _header: &physx_sys::PxContactPairHeader,
        _pairs: &[physx_sys::PxContactPair],
    ) {
    }
}
struct OnTrigger;
impl TriggerCallback for OnTrigger {
    fn on_trigger(&mut self, _pairs: &[physx_sys::PxTriggerPair]) {}
}

struct OnConstraintBreak;
impl ConstraintBreakCallback for OnConstraintBreak {
    fn on_constraint_break(&mut self, _constraints: &[physx_sys::PxConstraintInfo]) {}
}
struct OnWakeSleep;
impl WakeSleepCallback<PxArticulationLink, PxRigidStatic, PxRigidDynamic> for OnWakeSleep {
    fn on_wake_sleep(
        &mut self,
        _actors: &[&physx::actor::ActorMap<PxArticulationLink, PxRigidStatic, PxRigidDynamic>],
        _is_waking: bool,
    ) {
    }
}

struct OnAdvance;
impl AdvanceCallback<PxArticulationLink, PxRigidDynamic> for OnAdvance {
    fn on_advance(
        &self,
        _actors: &[&physx::rigid_body::RigidBodyMap<PxArticulationLink, PxRigidDynamic>],
        _transforms: &[PxTransform],
    ) {
    }
}

const DRIVABLE_SURFACE: u32 = 0xffff0000;
//const UNDRIVABLE_SURFACE: u32 = 0x0000ffff;

const COLLISION_FLAG_GROUND: u32 = 1 << 0;
const COLLISION_FLAG_WHEEL: u32 = 1 << 1;
const COLLISION_FLAG_CHASSIS: u32 = 1 << 2;
const COLLISION_FLAG_OBSTACLE: u32 = 1 << 3;
const COLLISION_FLAG_DRIVABLE_OBSTACLE: u32 = 1 << 4;

const COLLISION_FLAG_GROUND_AGAINST: u32 = COLLISION_FLAG_CHASSIS | COLLISION_FLAG_OBSTACLE | COLLISION_FLAG_DRIVABLE_OBSTACLE;
const COLLISION_FLAG_WHEEL_AGAINST: u32 = COLLISION_FLAG_WHEEL | COLLISION_FLAG_CHASSIS | COLLISION_FLAG_OBSTACLE;
const COLLISION_FLAG_CHASSIS_AGAINST: u32 = COLLISION_FLAG_GROUND | COLLISION_FLAG_WHEEL | COLLISION_FLAG_CHASSIS | COLLISION_FLAG_OBSTACLE | COLLISION_FLAG_DRIVABLE_OBSTACLE;
const COLLISION_FLAG_OBSTACLE_AGAINST: u32 = COLLISION_FLAG_GROUND | COLLISION_FLAG_WHEEL | COLLISION_FLAG_CHASSIS | COLLISION_FLAG_OBSTACLE | COLLISION_FLAG_DRIVABLE_OBSTACLE;
const COLLISION_FLAG_DRIVABLE_OBSTACLE_AGAINST: u32 = COLLISION_FLAG_GROUND | COLLISION_FLAG_CHASSIS | COLLISION_FLAG_OBSTACLE | COLLISION_FLAG_DRIVABLE_OBSTACLE;


fn main() {
    // Holds a PxFoundation and a PxPhysics.
    // Also has an optional Pvd and transport, not enabled by default.
    // The default allocator is the one provided by PhysX.


    //let mut physics = PhysicsFoundation::<_, PxShape>::default();
    //let mut physics = PhysicsFoundationBuilder::default().build().unwrap();


    let mut builder = PhysicsFoundationBuilder::default();
    builder.enable_visual_debugger(true);
    let mut physics: PhysicsFoundation<physx::foundation::DefaultAllocator, PxShape> =
        builder.build().expect("a foundation being built");

    utils::vehicle_sdk_init(physics.physics_mut());

    //let mut vd = VisualDebugger::new(physics.foundation_mut(), 5425).unwrap();
    //dbg!(vd.is_connected(false));
    //dbg!(vd.connect(PxPvdInstrumentationFlags { mBits: PxPvdInstrumentationFlag::eALL as u8 }));


    // Setup the scene object.  The PxScene type alias makes this much cleaner.
    // There are lots of unwrap calls due to potential null pointers.
    let mut scene: Owner<PxScene> = physics
        .create(SceneDescriptor {
            gravity: PxVec3::new(0.0, -9.81, 0.0),
            //on_advance: Some(OnAdvance),
            ..SceneDescriptor::new(())
        })
        .unwrap();

    let mut material = physics.create_material(0.5, 0.5, 0.6, ()).unwrap();

    create_drivable_plane(physics.physics_mut(), &mut material, &mut scene);

    /*let cyl = PxCapsuleGeometry::new(1.0, 10.0);

    let mut cyl_actor = physics
    .create_rigid_dynamic(
        PxTransform::from_translation(&PxVec3::new::new(0.0, 40.0, 100.0)),
        &cyl,
        material.as_mut(),
        10.0,
        PxTransform::default(),
        (),
    )
    .unwrap();
    cyl_actor.set_angular_damping(0.5);
    cyl_actor.set_rigid_body_flag(RigidBodyFlag::EnablePoseIntegrationPreview, true);
    scene.add_dynamic_actor(cyl_actor);*/

//    PxRigidActorExt_createExclusiveShape_mut(actor, geometry, materials, materialCount, shapeFlags)

//    cooking::PxCooking::new(foundation, params
/*/
    let data = [PxVec3::new(0.,1.,0.),PxVec3::new(1.,0.,0.),PxVec3::new(-1.,0.,0.),PxVec3::new(0.,0.,1.),PxVec3::new(0.,0.,-1.) ];

    let mut md = unsafe { PxConvexMeshDesc_new() };
    md.points.count = 5;
    md.points.stride = std::mem::size_of::<PxVec3>() as u32;
    md.points.data = data.as_ptr() as *const c_void;
    //md.indices.data = data.as_ptr() as *const c_void;
    //md.polygons.data = data.as_ptr() as *const c_void;
    md.quantizedCount = 10;
    md.flags = PxConvexFlags { mBits: PxConvexFlag::eCOMPUTE_CONVEX as u16 };

    let c = cooking::PxCooking::new(physics.foundation_mut(),
        &cooking::PxCookingParams { obj: unsafe { PxCookingParams_new(&PxTolerancesScale_new()) }}
    ).unwrap();
    dbg!(c.validate_convex_mesh(&PxConvexMeshDesc { obj: md }));
    let ConvexMeshCookingResult::Success(mesh) = c.create_convex_mesh(&mut physics, &PxConvexMeshDesc::default()) else { panic!(); };
    //let mesh = ConvexMeshCookingResult::Success(())
    //let x = ConvexMeshGeometry::new(mesh, &PxVec3::new::new(1., 1., 1.).into(), ConvexMeshGeometryFlag::TightBounds.into());


//    physics.create_convex_mesh(stream);
*/

    /*let sphere_geo = PxSphereGeometry::new(10.0);

    let mut sphere_actor = physics
        .create_rigid_dynamic(
            PxTransform::from_translation(&PxVec3::new(0.0, 40.0, 100.0)),
            &sphere_geo,
            material.as_mut(),
            10.0,
            PxTransform::default(),
            (),
        )
        .unwrap();
    sphere_actor.set_angular_damping(0.5);
    sphere_actor.set_rigid_body_flag(RigidBodyFlag::EnablePoseIntegrationPreview, true);
    scene.add_dynamic_actor(sphere_actor);*/

    fn create_drivable_plane(physics: &mut PxPhysics<PxShape>, material: &mut PxMaterial, scene: &mut PxScene) {
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
    }

    fn create_hull_mesh(physics: &mut PxPhysics<PxShape>, cooking: &PxCooking) -> Owner<ConvexMesh> {
        let vertices = HULL_VERTICES.iter().map(|v| PxVec3::new(v.x, v.y, v.z)).collect::<Vec<_>>();

        utils::create_convex_mesh(&vertices, physics, cooking)
    }

    fn get_wheel_offset(wheel: usize) -> PxVec3 {
        let v = WHEEL_OFFSETS[wheel];
        PxVec3::new(v.x, v.y, v.z)
    }

    fn create_vehicle(physics: &mut PxPhysics<PxShape>, cooking: &PxCooking, scene: &mut PxScene) -> *mut PxVehicleNoDrive {
        let material = physics.create_material(0.5, 0.5, 0.6, ()).unwrap();
        let mut veh_actor = physics.create_dynamic(&PxTransform::from_translation(&PxVec3::new(0.,0.,0.)), ()).unwrap();

        let wheel_sim_data = unsafe { PxVehicleWheelsSimData_allocate_mut(4).as_mut().unwrap() };

        let mut suspension_spring_masses = [0f32; 4];
        unsafe {
            let wheel_offsets = WHEEL_OFFSETS.iter().map(|v| PxVec3::new(v.x, v.y, v.z).into()).collect::<Vec<_>>();
            phys_PxVehicleComputeSprungMasses(4, wheel_offsets.as_ptr(), PxVec3::new(0., 0., 0.).as_ptr(), HULL_MASS, 1, suspension_spring_masses.as_mut_ptr());
        }

        for idx in 0..4 {
            let mut wheel = utils::create_cylinder_mesh(WHEEL_WIDTH, WHEEL_RADIUS, WHEEL_SEGMENTS, physics, cooking);
            let geom = utils::convex_mesh_to_geom(&mut wheel);

            unsafe {
                assert!(!PxRigidActorExt_createExclusiveShape_mut_1(
                    veh_actor.as_mut() as *mut PxRigidDynamic as *mut PxRigidActor,
                    &geom as *const PxConvexMeshGeometry as *const PxGeometry,
                    material.as_ptr(),
                    PxShapeFlags { mBits: (PxShapeFlag::eVISUALIZATION | PxShapeFlag::eSCENE_QUERY_SHAPE | PxShapeFlag::eSIMULATION_SHAPE) as u8 },
                ).is_null());
            }

            /*veh_actor.

            PxConvexMeshGeometry geom(wheelConvexMeshes[i]);
            PxShape* wheelShape = PxRigidActorExt::createExclusiveShape(*vehActor, geom, *wheelMaterials[i]);
            wheelShape->setQueryFilterData(wheelQryFilterData);
            //wheelShape->setSimulationFilterData(wheelSimFilterData);
            wheelShape->setLocalPose(PxTransform(PxIdentity));*/

            let mut wheel_data = unsafe { PxVehicleWheelData_new() };
            wheel_data.mMass = WHEEL_MASSES[idx];
            wheel_data.mMOI = 0.5 * WHEEL_MASSES[idx] * WHEEL_RADIUS * WHEEL_RADIUS;
            wheel_data.mRadius = WHEEL_RADIUS;
            wheel_data.mWidth = WHEEL_WIDTH;

            let mut tire_data = unsafe { PxVehicleTireData_new() };
            tire_data.mType = 0;

            let mut suspension = unsafe { PxVehicleSuspensionData_new() };
            suspension.mMaxCompression = 0.3;
            suspension.mMaxDroop = 0.1;
            suspension.mSpringStrength = 35000.;
            suspension.mSpringDamperRate = 4500.;
            suspension.mSprungMass = suspension_spring_masses[idx];

            let cmoffset = get_wheel_offset(idx);

            unsafe {
                PxVehicleWheelsSimData_setWheelData_mut(wheel_sim_data, idx as u32, &wheel_data as *const _);
                PxVehicleWheelsSimData_setTireData_mut(wheel_sim_data, idx as u32, &tire_data as *const _);
                PxVehicleWheelsSimData_setSuspensionData_mut(wheel_sim_data, idx as u32, &suspension as *const _);
                PxVehicleWheelsSimData_setSuspTravelDirection_mut(wheel_sim_data, idx as u32, PxVec3::new(0., -1., 0.).as_ptr());
                PxVehicleWheelsSimData_setWheelCentreOffset_mut(wheel_sim_data, idx as u32, cmoffset.as_ptr());
                PxVehicleWheelsSimData_setSuspForceAppPointOffset_mut(wheel_sim_data, idx as u32, PxVec3::new(cmoffset.x(), -0.3, cmoffset.z()).as_ptr());
                PxVehicleWheelsSimData_setTireForceAppPointOffset_mut(wheel_sim_data, idx as u32, PxVec3::new(cmoffset.x(), -0.3, cmoffset.z()).as_ptr());
                //PxVehicleWheelsSimData_setSceneQueryFilterData_mut(wheel_sim_data, idx as u32, sqFilterData);
                PxVehicleWheelsSimData_setWheelShapeMapping_mut(wheel_sim_data, idx as u32, idx as i32);
            }
        }

        let mut hull = create_hull_mesh(physics, cooking);
        let geom = utils::convex_mesh_to_geom(&mut hull);

        unsafe {
            assert!(!PxRigidActorExt_createExclusiveShape_mut_1(
                veh_actor.as_mut() as *mut PxRigidDynamic as *mut PxRigidActor,
                &geom as *const PxConvexMeshGeometry as *const PxGeometry,
                material.as_ptr(),
                PxShapeFlags { mBits: (PxShapeFlag::eVISUALIZATION | PxShapeFlag::eSCENE_QUERY_SHAPE | PxShapeFlag::eSIMULATION_SHAPE) as u8 },
            ).is_null());
        }

        veh_actor.set_mass(HULL_MASS);

        let moi = PxVec3::new(
            (HULL_DIMENSIONS.y * HULL_DIMENSIONS.y + HULL_DIMENSIONS.z * HULL_DIMENSIONS.z) * HULL_MASS / 12.0,
            (HULL_DIMENSIONS.x * HULL_DIMENSIONS.x + HULL_DIMENSIONS.z * HULL_DIMENSIONS.z) * 0.8 * HULL_MASS / 12.0,
            (HULL_DIMENSIONS.x * HULL_DIMENSIONS.x + HULL_DIMENSIONS.y * HULL_DIMENSIONS.y) * HULL_MASS / 12.0,
        );
        veh_actor.set_mass_space_inertia_tensor(&moi);

        veh_actor.set_c_mass_local_pose(&PxTransform::from_translation(&PxVec3::new(HULL_CENTER.x, HULL_CENTER.y, HULL_CENTER.z)));
        //veh_actor.set_global_pose(&PxTransform::from_translation(&PxVec3::new(0., 100., 0.)), true);

        let vehicle = unsafe { PxVehicleNoDrive_allocate_mut(4) };
        unsafe {PxVehicleNoDrive_setup_mut(vehicle, physics.as_mut_ptr(), veh_actor.as_mut_ptr(), wheel_sim_data); }

        scene.add_dynamic_actor(veh_actor);

        unsafe {
            PxVehicleNoDrive_setDriveTorque_mut(vehicle, 2, -1000.);
            PxVehicleNoDrive_setDriveTorque_mut(vehicle, 3, -1000.);
        }

        vehicle
    }

    let params = &cooking::PxCookingParams::new(&physics).expect("failed to create cooking params");
    let gcooking = cooking::PxCooking::new(physics.foundation_mut(), &params).expect("failed to create cooking");
/*
    let mut xx = create_cylinder_mesh(10., 10., 16, physics.physics_mut(), gcooking.as_ref());

    let geom = PxConvexMeshGeometry::new(&mut xx, unsafe { &PxMeshScale_new() }, PxConvexMeshGeometryFlags { mBits: PxConvexMeshGeometryFlag::eTIGHT_BOUNDS as u8 });

    let cyl = PxCapsuleGeometry::new(1.0, 10.0);
    let mut body = physics.create_rigid_dynamic(
        PxTransform::from_translation(&PxVec3::new(0.0, 40.0, 100.0)),
        &geom,
        material.as_mut(),
        10.0,
        PxTransform::default(),
        (),
    ).expect("failed to create rigid dynamic");*/


    /*let wheel_shape = unsafe {
        PxRigidActorExt_createExclusiveShape_mut_1(
            body.as_mut() as *mut PxRigidDynamic as *mut PxRigidActor,
            &geom as *const PxConvexMeshGeometry as *const PxGeometry,
            material.as_ptr(),
            PxShapeFlags { mBits: (PxShapeFlag::eVISUALIZATION | PxShapeFlag::eSCENE_QUERY_SHAPE | PxShapeFlag::eSIMULATION_SHAPE) as u8 },
        )
    };
    unsafe {
        PxRigidBodyExt_updateMassAndInertia_mut_1(
            body.as_mut() as *mut PxRigidDynamic as *mut PxRigidBody,
            10.,
            null(),
            false,
        );
    }*/
    //scene.add_dynamic_actor(body);



    let mut xx = utils::create_cylinder_mesh(1., 1., 16, physics.physics_mut(), gcooking.as_ref());

    let geom = PxConvexMeshGeometry::new(&mut xx, unsafe { &PxMeshScale_new() }, PxConvexMeshGeometryFlags { mBits: PxConvexMeshGeometryFlag::eTIGHT_BOUNDS as u8 });

    let cyl = PxCapsuleGeometry::new(1.0, 10.0);
    let mut body = physics.create_rigid_dynamic(
        PxTransform::from_translation(&PxVec3::new(0.0, 1.0, -40.0)),
        &geom,
        material.as_mut(),
        10.0,
        PxTransform::default(),
        (),
    ).expect("failed to create rigid dynamic");
    scene.add_dynamic_actor(body);

    extern "C" fn pre_filter_shader(data0: PxFilterData, data1: PxFilterData, cblock: c_void, cblocksize: u32, flags: PxHitFlags) -> u32 {
        println!("{} {} {} {}", data1.word0, data1.word1, data1.word2, data1.word3);

        if 0 == (data1.word3 & DRIVABLE_SURFACE) {
            PxQueryHitType::eNONE
        } else {
            PxQueryHitType::eBLOCK
        }
    }

    let mut sq_results = [0u8; 80 * 4]; // PxRaycastQueryResult, rust port generates wrong struct size
    let mut sq_hit_buffer: MaybeUninit<[ PxRaycastHit; 4 ]> = MaybeUninit::uninit();
    let mut sq_desc = unsafe { PxBatchQueryDesc_new(4, 0, 0) };

    let mut vehicles = [ create_vehicle(physics.physics_mut(), gcooking.as_ref(), scene.as_mut()) ];
    let batch_query = unsafe {
        sq_desc.queryMemory.userRaycastResultBuffer = sq_results.as_mut_ptr() as *mut PxRaycastQueryResult;
        sq_desc.queryMemory.userRaycastTouchBuffer = sq_hit_buffer.as_mut_ptr() as *mut PxRaycastHit;
        sq_desc.queryMemory.raycastTouchBufferSize = 4;
        sq_desc.preFilterShader = pre_filter_shader as *mut c_void;
        PxScene_createBatchQuery_mut(scene.as_mut_ptr(), &sq_desc as *const _)
    };

    let friction_pairs = unsafe { PxVehicleDrivableSurfaceToTireFrictionPairs_allocate_mut(1, 1) };
    unsafe {
        let mut surface_types = [ PxVehicleDrivableSurfaceType { mType: 0 } ];
        let mut surface_materials : [ *const physx_sys::PxMaterial; 1 ] = [ material.as_ptr() ];

        PxVehicleDrivableSurfaceToTireFrictionPairs_setup_mut(friction_pairs, 1, 1, surface_materials.as_mut_ptr(), surface_types.as_mut_ptr());
        PxVehicleDrivableSurfaceToTireFrictionPairs_setTypePairFriction_mut(friction_pairs, 0, 0, 1000.);
    };

    for _ in 0..2342 {
        unsafe {
            phys_PxVehicleSuspensionRaycasts(
                batch_query,
                1,
                vehicles.as_mut_ptr() as *mut *mut PxVehicleWheels,
                4,
                sq_results.as_mut_ptr() as *mut PxRaycastQueryResult,
                [ true; 1 ].as_ptr(),
            );

            phys_PxVehicleUpdates(
                1.0 / 60.0,
                PxVec3::new(0.0, -9.81, 0.0).as_ptr(),
                friction_pairs,
                1,
                vehicles.as_mut_ptr() as *mut *mut PxVehicleWheels,
                null_mut(),
                null_mut(),
            );
        }

        scene.simulate(1./60., None, None);
        scene.fetch_results(true).unwrap();
    }

    println!("simulation done");

    utils::vehicle_sdk_done(physics.physics_mut());
}
