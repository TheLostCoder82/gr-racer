use crate::engine::ecs::{World, Entity, Transform, Camera, PointLight, DirectionalLight, MeshPrimitive};
use glam::{Vec3, Quat};
use rand::Rng;

/// Creates a demo scene with ground, primitives, lights, and camera
pub fn create_demo_scene(world: &mut World) {
    let mut rng = rand::thread_rng();

    // Create ground plane (static rigid body would be added in physics integration)
    let ground_entity = world.create_entity();
    let ground_transform = Transform {
        position: Vec3::new(0.0, -1.0, 0.0),
        rotation: Quat::IDENTITY,
        scale: Vec3::new(20.0, 0.1, 20.0),
    };
    world.insert_component(ground_entity, ground_transform);
    world.insert_component(ground_entity, MeshPrimitive::Plane);

    // Create directional light (sun)
    let sun_entity = world.create_entity();
    let sun_light = DirectionalLight::new(
        Vec3::new(-1.0, -2.0, -1.0).normalize(),
        Vec3::new(1.0, 0.95, 0.9),
        1.0,
    );
    world.insert_component(sun_entity, sun_light);

    // Create point light
    let point_light_entity = world.create_entity();
    let mut point_light = PointLight::new(Vec3::new(1.0, 0.8, 0.5), 2.0);
    point_light.position = Vec3::new(0.0, 3.0, 0.0);
    world.insert_component(point_light_entity, point_light);

    // Create camera entity
    let camera_entity = world.create_entity();
    let camera = Camera::perspective(1.2, 16.0 / 9.0, 0.1, 100.0);
    let camera_transform = Transform {
        position: Vec3::new(0.0, 2.0, 8.0),
        rotation: Quat::from_rotation_y(std::f32::consts::PI),
        scale: Vec3::ONE,
    };
    world.insert_component(camera_entity, camera);
    world.insert_component(camera_entity, camera_transform);

    // Create 8 dynamic primitives (cubes and spheres)
    let colors = [
        Vec3::new(1.0, 0.0, 0.0),   // Red
        Vec3::new(0.0, 1.0, 0.0),   // Green
        Vec3::new(0.0, 0.0, 1.0),   // Blue
        Vec3::new(1.0, 1.0, 0.0),   // Yellow
        Vec3::new(1.0, 0.0, 1.0),   // Magenta
        Vec3::new(0.0, 1.0, 1.0),   // Cyan
        Vec3::new(1.0, 0.5, 0.0),   // Orange
        Vec3::new(0.5, 0.0, 1.0),   // Purple
    ];

    for i in 0..8 {
        let primitive_entity = world.create_entity();
        
        let x = (rng.gen::<f32>() - 0.5) * 10.0;
        let y = 2.0 + rng.gen::<f32>() * 3.0;
        let z = (rng.gen::<f32>() - 0.5) * 10.0;

        let transform = Transform {
            position: Vec3::new(x, y, z),
            rotation: Quat::from_axis_angle(Vec3::Y, rng.gen::<f32>() * std::f32::consts::PI),
            scale: Vec3::splat(0.5 + rng.gen::<f32>() * 0.5),
        };

        world.insert_component(primitive_entity, transform);

        // Alternate between cubes and spheres
        if i % 2 == 0 {
            world.insert_component(primitive_entity, MeshPrimitive::Cube);
        } else {
            world.insert_component(primitive_entity, MeshPrimitive::Sphere);
        }
    }
}

/// Demo scene runner
pub struct DemoScene {
    pub is_active: bool,
}

impl DemoScene {
    pub fn new() -> Self {
        Self { is_active: true }
    }

    pub fn setup(&self, world: &mut World) {
        create_demo_scene(world);
    }
}

impl Default for DemoScene {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_scene_creation() {
        let mut world = World::new();
        create_demo_scene(&mut world);

        // Verify entities were created
        // We should have: ground, sun, point light, camera, 8 primitives = 12 entities
        assert!(world.iter_entities_with_components(&["Transform"]).len() >= 10);
        
        // Verify we have at least one camera
        let cameras = world.iter_entities_with_components(&["Camera"]);
        assert!(!cameras.is_empty());

        // Verify we have lights
        let point_lights = world.iter_entities_with_components(&["PointLight"]);
        let dir_lights = world.iter_entities_with_components(&["DirectionalLight"]);
        assert!(!point_lights.is_empty() || !dir_lights.is_empty());
    }

    #[test]
    fn test_demo_scene_primitives() {
        let mut world = World::new();
        create_demo_scene(&mut world);

        let meshes = world.iter_entities_with_components(&["MeshPrimitive"]);
        assert!(meshes.len() >= 8); // Ground + 8 primitives
    }
}
