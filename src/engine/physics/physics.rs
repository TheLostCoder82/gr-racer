use rapier3d::prelude::*;

/// Physics world wrapper
pub struct PhysicsWorld {
    pub gravity: Vector<f32>,
    pub integration_parameters: IntegrationParameters,
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub impulse_joints: ImpulseJointSet,
    pub multibody_joints: MultibodyJointSet,
    pub pipeline: PhysicsPipeline,
    pub query_pipeline: QueryPipeline,
}

impl PhysicsWorld {
    pub fn new() -> Self {
        let gravity = Vector::new(0.0, -9.81, 0.0);
        
        Self {
            gravity,
            integration_parameters: IntegrationParameters::default(),
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            impulse_joints: ImpulseJointSet::new(),
            multibody_joints: MultibodyJointSet::new(),
            pipeline: PhysicsPipeline::new(),
            query_pipeline: QueryPipeline::new(),
        }
    }

    pub fn step(&mut self, dt: f32) {
        self.pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joints,
            &mut self.multibody_joints,
            &mut self.query_pipeline,
            &(),
        );
    }

    pub fn create_rigid_body(&mut self, body_type: RigidBodyType) -> RigidBodyHandle {
        let builder = RigidBodyBuilder::new(body_type);
        self.rigid_body_set.insert(builder.build())
    }

    pub fn create_collider(
        &mut self,
        shape: SharedShape,
        body_handle: Option<RigidBodyHandle>,
    ) -> ColliderHandle {
        let builder = ColliderBuilder::new(shape);
        
        if let Some(handle) = body_handle {
            self.collider_set.insert_with_parent(builder.build(), handle, &mut self.rigid_body_set)
        } else {
            self.collider_set.insert(builder.build())
        }
    }

    pub fn get_rigid_body(&self, handle: RigidBodyHandle) -> Option<&RigidBody> {
        self.rigid_body_set.get(handle)
    }

    pub fn get_rigid_body_mut(&mut self, handle: RigidBodyHandle) -> Option<&mut RigidBody> {
        self.rigid_body_set.get_mut(handle)
    }

    pub fn update_query_pipeline(&mut self) {
        self.query_pipeline.update(&self.rigid_body_set, &self.collider_set);
    }
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self::new()
    }
}

/// Physics sync system - syncs ECS transforms with Rapier bodies
pub struct PhysicsSyncSystem;

impl PhysicsSyncSystem {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PhysicsSyncSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physics_world_gravity() {
        let world = PhysicsWorld::new();
        assert_eq!(world.gravity, Vector::new(0.0, -9.81, 0.0));
    }

    #[test]
    fn test_create_rigid_body() {
        let mut world = PhysicsWorld::new();
        let handle = world.create_rigid_body(RigidBodyType::Dynamic);
        assert!(world.get_rigid_body(handle).is_some());
    }

    #[test]
    fn test_create_static_collider() {
        let mut world = PhysicsWorld::new();
        let shape = SharedShape::cuboid(1.0, 1.0, 1.0);
        let handle = world.create_collider(shape, None);
        assert!(world.collider_set.get(handle).is_some());
    }
}
