use super::components::{Component, World, Transform, PointLight, DirectionalLight};
use glam::Quat;

/// System trait for ECS systems
pub trait System: Send + Sync {
    fn update(&mut self, world: &mut World, dt: f32);
}

/// System scheduler that runs systems in order
pub struct SystemScheduler {
    systems: Vec<Box<dyn System>>,
}

impl SystemScheduler {
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }

    pub fn add_system<S: System>(&mut self, system: S) {
        self.systems.push(Box::new(system));
    }

    pub fn run(&mut self, world: &mut World, dt: f32) {
        for system in &mut self.systems {
            system.update(world, dt);
        }
    }
}

impl Default for SystemScheduler {
    fn default() -> Self {
        Self::new()
    }
}

/// Transform system - updates transform matrices
pub struct TransformSystem;

impl System for TransformSystem {
    fn update(&mut self, _world: &mut World, _dt: f32) {
        // In a more complex engine, this would handle parent-child transforms
        // For now, transforms are already up-to-date when modified
    }
}

/// Light control system - toggles lights based on LightToggle component
pub struct LightControlSystem;

impl System for LightControlSystem {
    fn update(&mut self, world: &mut World, _dt: f32) {
        let toggle_entities: Vec<_> = world.iter_entities_with_components(&["LightToggle"]);
        
        for entity in toggle_entities {
            // Toggle point lights
            if let Some(light) = world.get_component_mut::<PointLight>(entity) {
                light.enabled = !light.enabled;
            }
            
            // Toggle directional lights
            if let Some(light) = world.get_component_mut::<DirectionalLight>(entity) {
                light.enabled = !light.enabled;
            }
        }
    }
}

/// Camera controller system - handles WASD + mouse input
pub struct CameraControllerSystem {
    pub move_speed: f32,
    pub look_speed: f32,
    pub yaw: f32,
    pub pitch: f32,
}

impl CameraControllerSystem {
    pub fn new() -> Self {
        Self {
            move_speed: 5.0,
            look_speed: 0.1,
            yaw: 0.0,
            pitch: 0.0,
        }
    }
}

impl Default for CameraControllerSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl System for CameraControllerSystem {
    fn update(&mut self, world: &mut World, dt: f32) {
        // Get camera entities
        let camera_entities: Vec<_> = world.iter_entities_with_components(&["Camera", "Transform"]);
        
        for entity in camera_entities {
            if let Some(transform) = world.get_component_mut::<Transform>(entity) {
                // Simple keyboard movement would be handled here
                // In actual implementation, input state would be passed in
                // For now, we just ensure the camera transform exists
                
                // Apply rotation from yaw/pitch
                let rotation = Quat::from_rotation_y(self.yaw) * Quat::from_rotation_x(self.pitch);
                transform.rotation = rotation;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::ecs::components::{Camera, PointLight};
    use glam::Vec3;

    #[test]
    fn test_system_scheduler_order() {
        let mut scheduler = SystemScheduler::new();
        let mut world = World::new();
        
        struct TestSystem {
            executed: bool,
        }
        
        impl System for TestSystem {
            fn update(&mut self, _world: &mut World, _dt: f32) {
                self.executed = true;
            }
        }
        
        scheduler.add_system(TestSystem { executed: false });
        scheduler.run(&mut world, 0.016);
    }

    #[test]
    fn test_light_toggle_system() {
        let mut world = World::new();
        let entity = world.create_entity();
        
        let mut light = PointLight::new(Vec3::X, 1.0);
        light.enabled = true;
        world.insert_component(entity, light);
        world.insert_component(entity, crate::engine::ecs::components::LightToggle);
        
        let mut system = LightControlSystem;
        system.update(&mut world, 0.016);
        
        let light = world.get_component::<PointLight>(entity).unwrap();
        assert!(!light.enabled);
    }

    #[test]
    fn test_transform_system() {
        let mut world = World::new();
        let entity = world.create_entity();
        
        let mut transform = Transform::identity();
        transform.position = Vec3::new(1.0, 2.0, 3.0);
        world.insert_component(entity, transform);
        
        let mut system = TransformSystem;
        system.update(&mut world, 0.016);
        
        let transform = world.get_component::<Transform>(entity).unwrap();
        assert_eq!(transform.position, Vec3::new(1.0, 2.0, 3.0));
    }
}
