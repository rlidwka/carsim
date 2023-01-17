use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use physx::cooking::{self, PxCooking};
use physx::prelude::*;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

mod type_bridge;
use type_bridge::*;

mod utils;
mod systems;

pub mod callbacks;
pub mod prelude;

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
    callbacks::OnCollision,
    callbacks::OnTrigger,
    callbacks::OnConstraintBreak,
    callbacks::OnWakeSleep,
    callbacks::OnAdvance,
>;

pub struct BPxPlugin {
    pub vehicles: bool,
    pub cooking: bool,
    pub debugger: bool,
    pub gravity: Vec3,
}

impl Plugin for BPxPlugin {
    fn build(&self, app: &mut App) {
        let mut builder = physx::physics::PhysicsFoundationBuilder::default();
        builder.enable_visual_debugger(self.debugger);

        let mut physics = builder.build().expect("building PhysX foundation failed");

        if self.vehicles {
            utils::vehicle_sdk_init(physics.physics_mut());
        }

        let scene = physics
            .create(SceneDescriptor {
                gravity: self.gravity.to_physx(),
                ..SceneDescriptor::new(())
            })
            .unwrap();

        if self.cooking {
            let params = &cooking::PxCookingParams::new(&physics).expect("failed to create cooking params");
            let cooking = cooking::PxCooking::new(physics.foundation_mut(), &params).expect("failed to create cooking");
            app.insert_resource(BPxCooking(cooking));
        }

        app.add_asset::<BPxGeometry>();
        app.add_asset::<BPxMaterial>();

        app.insert_resource(BPxScene(scene));

        // physics must be last (so it will be dropped last)
        app.insert_resource(BPxPhysics { physics, vsdk: self.vehicles });

        #[derive(Debug, StageLabel)]
        struct PhysXStage;

        let mut stage = SystemStage::parallel();
        stage.add_system(systems::scene_simulate);
        stage.add_system(systems::create_dynamic_actors.after(systems::scene_simulate));
        stage.add_system(systems::writeback_actors.after(systems::scene_simulate));

        app.add_stage_after(CoreStage::Update, PhysXStage, stage);
    }
}

impl Default for BPxPlugin {
    fn default() -> Self {
        Self {
            vehicles: false,
            cooking: false,
            debugger: false,
            gravity: Vec3::new(0.0, -9.81, 0.0),
        }
    }
}

#[derive(Resource)]
pub struct BPxPhysics {
    physics: PhysicsFoundation<physx::foundation::DefaultAllocator, PxShape>,
    vsdk: bool,
}

impl Deref for BPxPhysics {
    type Target = PhysicsFoundation<physx::foundation::DefaultAllocator, PxShape>;
    fn deref(&self) -> &Self::Target {
        &self.physics
    }
}

impl DerefMut for BPxPhysics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.physics
    }
}

impl Drop for BPxPhysics {
    fn drop(&mut self) {
        if self.vsdk {
            utils::vehicle_sdk_done();
            return;
        }
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct BPxScene(Owner<PxScene>);

#[derive(Resource, Deref, DerefMut)]
pub struct BPxCooking(Owner<PxCooking>);

#[derive(Component)]
pub struct BPxRigidDynamic {
    pub geometry: Handle<BPxGeometry>,
    pub material: Handle<BPxMaterial>,
    pub density: f32,
    pub shape_transform: Transform,
}

impl Default for BPxRigidDynamic {
    fn default() -> Self {
        Self {
            geometry: Default::default(),
            material: Default::default(),
            density: 1.0,
            shape_transform: Transform::IDENTITY,
        }
    }
}

#[derive(TypeUuid, Deref, DerefMut)]
#[uuid = "5351ec05-c0fd-426a-b35e-62008a6b10e1"]
pub struct BPxMaterial(Owner<PxMaterial>);

impl From<Owner<PxMaterial>> for BPxMaterial {
    fn from(value: Owner<PxMaterial>) -> Self {
        BPxMaterial(value)
    }
}

#[derive(TypeUuid, Deref, DerefMut)]
#[uuid = "db246120-e6af-4ebf-a95a-a6efe1c54d9f"]
pub struct BPxGeometry(Box<dyn Geometry + Send + Sync>);

impl<T> From<T> for BPxGeometry where T: Geometry + Send + Sync + 'static {
    fn from(value: T) -> Self {
        BPxGeometry(Box::new(value))
    }
}
