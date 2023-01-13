use std::f32::consts::PI;
use std::ffi::c_void;

use physx::convex_mesh::ConvexMesh;
use physx::cooking::ConvexMeshCookingResult;
use physx::cooking::PxConvexMeshDesc;
use physx::cooking::PxCooking;
//use physx::cooking::PxTriangleMeshDesc;
//use physx::cooking::TriangleMeshCookingResult;
use physx::physics::PxPhysics;
use physx::prelude::*;
//use physx::triangle_mesh::TriangleMesh;
use physx_sys::PxConvexFlag;
use physx_sys::PxConvexFlags;
use physx_sys::PxConvexMeshGeometryFlag;
use physx_sys::PxConvexMeshGeometryFlags;
//use physx_sys::PxMeshGeometryFlag;
//use physx_sys::PxMeshGeometryFlags;
use physx_sys::PxMeshScale_new;


pub fn create_convex_mesh<Geom: Shape>(verts: &[PxVec3], physics: &mut PxPhysics<Geom>, cooking: &PxCooking) -> Owner<ConvexMesh> {
    let mut mesh_desc = PxConvexMeshDesc::new();
    mesh_desc.obj.points.count = verts.len() as u32;
    mesh_desc.obj.points.stride = std::mem::size_of::<PxVec3>() as u32;
    mesh_desc.obj.points.data = verts.as_ptr() as *const c_void;
    mesh_desc.obj.flags = PxConvexFlags { mBits: PxConvexFlag::eCOMPUTE_CONVEX as u16 };

    match cooking.create_convex_mesh(physics, &mesh_desc) {
        ConvexMeshCookingResult::Success(mesh) => mesh,
        ConvexMeshCookingResult::Failure => panic!("create_convex_mesh failure"),
        ConvexMeshCookingResult::InvalidDescriptor => panic!("create_convex_mesh invalid descriptor"),
        ConvexMeshCookingResult::PolygonsLimitReached => panic!("create_convex_mesh polygon limit reached"),
        ConvexMeshCookingResult::ZeroAreaTestFailed => panic!("create_convex_mesh zero area test failed"),
    }
}

/*pub fn create_triangle_mesh<Geom: Shape>(verts: &[PxVec3], indices: &[u32], physics: &mut PxPhysics<Geom>, cooking: &PxCooking) -> Owner<TriangleMesh> {
    let mut mesh_desc = PxTriangleMeshDesc::new();
    mesh_desc.obj.points.count = verts.len() as u32;
    mesh_desc.obj.points.stride = std::mem::size_of::<PxVec3>() as u32;
    mesh_desc.obj.points.data = verts.as_ptr() as *const c_void;

    assert_eq!(indices.len() % 3, 0);
    mesh_desc.obj.triangles.count = indices.len() as u32 / 3;
    mesh_desc.obj.triangles.stride = 3 * std::mem::size_of::<u32>() as u32;
    mesh_desc.obj.triangles.data = indices.as_ptr() as *const c_void;

    match cooking.create_triangle_mesh(physics, &mesh_desc) {
        TriangleMeshCookingResult::Success(mesh) => mesh,
        TriangleMeshCookingResult::Failure => panic!("create_triangle_mesh failure"),
        TriangleMeshCookingResult::InvalidDescriptor => panic!("create_triangle_mesh invalid descriptor"),
        TriangleMeshCookingResult::LargeTriangle => panic!("create_triangle_mesh large triangle"),
    }
}*/

pub fn create_cylinder_mesh<Geom: Shape>(width: f32, radius: f32, segments: usize, physics: &mut PxPhysics<Geom>, cooking: &PxCooking) -> Owner<ConvexMesh> {
    let mut points = [PxVec3::default(); 2 * 16];

    for i in 0..segments {
        let cos_theta = (i as f32 * PI * 2. / segments as f32).cos();
        let sin_theta = (i as f32 * PI * 2. / segments as f32).sin();
        let y = radius * cos_theta;
        let z = radius * sin_theta;
        points[2 * i + 0] = PxVec3::new(-width / 2., y, z);
        points[2 * i + 1] = PxVec3::new( width / 2., y, z);
    }

    create_convex_mesh(&points, physics, cooking)
}

pub fn convex_mesh_to_geom(mesh: &mut ConvexMesh) -> PxConvexMeshGeometry {
    PxConvexMeshGeometry::new(
        mesh,
        unsafe { &PxMeshScale_new() },
        PxConvexMeshGeometryFlags { mBits: PxConvexMeshGeometryFlag::eTIGHT_BOUNDS as u8 }
    )
}

/*pub fn triangle_mesh_to_geom(mesh: &mut TriangleMesh) -> PxTriangleMeshGeometry {
    PxTriangleMeshGeometry::new(
        mesh,
        unsafe { &PxMeshScale_new() },
        PxMeshGeometryFlags { mBits: PxMeshGeometryFlag::eDOUBLE_SIDED as u8 }
    )
}*/
