use std::ptr::null_mut;
use physx::physics::PxPhysics;
use physx::prelude::*;
use physx::traits::Class;
use physx_sys::PxVehicleUpdateMode;
use physx_sys::phys_PxCloseVehicleSDK;
use physx_sys::phys_PxInitVehicleSDK;
use physx_sys::phys_PxVehicleSetBasisVectors;
use physx_sys::phys_PxVehicleSetUpdateMode;


pub fn vehicle_sdk_init<Geom: Shape>(physics: &mut PxPhysics<Geom>) {
    unsafe {
        phys_PxInitVehicleSDK(physics.as_mut_ptr(), null_mut());
        phys_PxVehicleSetBasisVectors(PxVec3::new(0.,1.,0.).as_ptr(), PxVec3::new(0.,0.,1.).as_ptr());
        phys_PxVehicleSetUpdateMode(PxVehicleUpdateMode::eVELOCITY_CHANGE);
    }
}

pub fn vehicle_sdk_done() {
    unsafe {
        phys_PxCloseVehicleSDK(null_mut());
    }
}
