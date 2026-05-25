use std::collections::HashMap;
use glam::{Vec3, Quat, Mat4};

/// Component marker trait
pub trait Component: Send + Sync + 'static {}

/// Entity ID with generational index
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    pub id: u64,
    pub generation: u32,
}

impl Entity {
    pub fn new(id: u64, generation: u32) -> Self {
        Self { id, generation }
    }
}

/// Transform component for position, rotation, and scale
#[derive(Debug, Clone)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    pub fn identity() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    pub fn to_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }
}

impl Component for Transform {}

/// Camera component
#[derive(Debug, Clone)]
pub struct Camera {
    pub fov_y: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
    pub is_active: bool,
}

impl Camera {
    pub fn perspective(fov_y: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        Self {
            fov_y,
            aspect_ratio,
            near,
            far,
            is_active: true,
        }
    }

    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh_gl(self.fov_y, self.aspect_ratio, self.near, self.far)
    }

    pub fn view_matrix(&self, transform: &Transform) -> Mat4 {
        Mat4::look_at_rh(
            transform.position,
            transform.position + transform.rotation * Vec3::Z,
            Vec3::Y,
        )
    }
}

impl Component for Camera {}

/// Point light component
#[derive(Debug, Clone)]
pub struct PointLight {
    pub position: Vec3,
    pub color: Vec3,
    pub intensity: f32,
    pub enabled: bool,
}

impl PointLight {
    pub fn new(color: Vec3, intensity: f32) -> Self {
        Self {
            position: Vec3::ZERO,
            color,
            intensity,
            enabled: true,
        }
    }
}

impl Component for PointLight {}

/// Directional light component
#[derive(Debug, Clone)]
pub struct DirectionalLight {
    pub direction: Vec3,
    pub color: Vec3,
    pub intensity: f32,
    pub enabled: bool,
}

impl DirectionalLight {
    pub fn new(direction: Vec3, color: Vec3, intensity: f32) -> Self {
        Self {
            direction: direction.normalize(),
            color,
            intensity,
            enabled: true,
        }
    }
}

impl Component for DirectionalLight {}

/// Mesh primitive types
#[derive(Debug, Clone, Copy)]
pub enum MeshPrimitive {
    Cube,
    Sphere,
    Plane,
    Cylinder,
}

impl Component for MeshPrimitive {}

/// Light toggle marker for UI interaction
#[derive(Debug, Clone)]
pub struct LightToggle;

impl Component for LightToggle {}

/// ECS World
pub struct World {
    entities: HashMap<Entity, Vec<String>>,
    next_id: u64,
    generations: HashMap<u64, u32>,
    
    // Component storage (simplified - in production use better data structures)
    transforms: HashMap<Entity, Transform>,
    cameras: HashMap<Entity, Camera>,
    point_lights: HashMap<Entity, PointLight>,
    directional_lights: HashMap<Entity, DirectionalLight>,
    mesh_primitives: HashMap<Entity, MeshPrimitive>,
    light_toggles: HashMap<Entity, LightToggle>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            next_id: 0,
            generations: HashMap::new(),
            transforms: HashMap::new(),
            cameras: HashMap::new(),
            point_lights: HashMap::new(),
            directional_lights: HashMap::new(),
            mesh_primitives: HashMap::new(),
            light_toggles: HashMap::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        let id = self.next_id;
        self.next_id += 1;
        let generation = *self.generations.entry(id).or_insert(0);
        let entity = Entity::new(id, generation);
        self.entities.insert(entity, Vec::new());
        entity
    }

    pub fn insert_component<T: Component>(&mut self, entity: Entity, component: T) -> bool {
        if !self.entities.contains_key(&entity) {
            return false;
        }

        let type_name = std::any::type_name::<T>();
        
        match type_name {
            _ if type_name.contains("Transform") => {
                if let Some(comp) = (&component as &dyn std::any::Any).downcast_ref::<Transform>() {
                    self.transforms.insert(entity, comp.clone());
                }
            }
            _ if type_name.contains("Camera") => {
                if let Some(comp) = (&component as &dyn std::any::Any).downcast_ref::<Camera>() {
                    self.cameras.insert(entity, comp.clone());
                }
            }
            _ if type_name.contains("PointLight") => {
                if let Some(comp) = (&component as &dyn std::any::Any).downcast_ref::<PointLight>() {
                    self.point_lights.insert(entity, comp.clone());
                }
            }
            _ if type_name.contains("DirectionalLight") => {
                if let Some(comp) = (&component as &dyn std::any::Any).downcast_ref::<DirectionalLight>() {
                    self.directional_lights.insert(entity, comp.clone());
                }
            }
            _ if type_name.contains("MeshPrimitive") => {
                if let Some(comp) = (&component as &dyn std::any::Any).downcast_ref::<MeshPrimitive>() {
                    self.mesh_primitives.insert(entity, *comp);
                }
            }
            _ if type_name.contains("LightToggle") => {
                self.light_toggles.insert(entity, LightToggle);
            }
            _ => {}
        }

        if !self.entities.get(&entity).unwrap().contains(&type_name.to_string()) {
            self.entities.get_mut(&entity).unwrap().push(type_name.to_string());
        }

        true
    }

    pub fn get_component<T: Component>(&self, entity: Entity) -> Option<&T> {
        let type_name = std::any::type_name::<T>();
        
        match type_name {
            _ if type_name.contains("Transform") => {
                self.transforms.get(&entity).and_then(|c| (c as &dyn std::any::Any).downcast_ref::<T>())
            }
            _ if type_name.contains("Camera") => {
                self.cameras.get(&entity).and_then(|c| (c as &dyn std::any::Any).downcast_ref::<T>())
            }
            _ if type_name.contains("PointLight") => {
                self.point_lights.get(&entity).and_then(|c| (c as &dyn std::any::Any).downcast_ref::<T>())
            }
            _ if type_name.contains("DirectionalLight") => {
                self.directional_lights.get(&entity).and_then(|c| (c as &dyn std::any::Any).downcast_ref::<T>())
            }
            _ if type_name.contains("MeshPrimitive") => {
                self.mesh_primitives.get(&entity).and_then(|c| (c as &dyn std::any::Any).downcast_ref::<T>())
            }
            _ if type_name.contains("LightToggle") => {
                self.light_toggles.get(&entity).and_then(|c| (c as &dyn std::any::Any).downcast_ref::<T>())
            }
            _ => None,
        }
    }

    pub fn get_component_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T> {
        let type_name = std::any::type_name::<T>();
        
        match type_name {
            _ if type_name.contains("Transform") => {
                self.transforms.get_mut(&entity).and_then(|c| (c as &dyn std::any::Any).downcast_mut::<T>())
            }
            _ if type_name.contains("Camera") => {
                self.cameras.get_mut(&entity).and_then(|c| (c as &dyn std::any::Any).downcast_mut::<T>())
            }
            _ if type_name.contains("PointLight") => {
                self.point_lights.get_mut(&entity).and_then(|c| (c as &dyn std::any::Any).downcast_mut::<T>())
            }
            _ if type_name.contains("DirectionalLight") => {
                self.directional_lights.get_mut(&entity).and_then(|c| (c as &dyn std::any::Any).downcast_mut::<T>())
            }
            _ if type_name.contains("MeshPrimitive") => {
                self.mesh_primitives.get_mut(&entity).and_then(|c| (c as &dyn std::any::Any).downcast_mut::<T>())
            }
            _ if type_name.contains("LightToggle") => {
                self.light_toggles.get_mut(&entity).and_then(|c| (c as &dyn std::any::Any).downcast_mut::<T>())
            }
            _ => None,
        }
    }

    pub fn remove_component<T: Component>(&mut self, entity: Entity) -> bool {
        let type_name = std::any::type_name::<T>();
        
        match type_name {
            _ if type_name.contains("Transform") => {
                self.transforms.remove(&entity);
            }
            _ if type_name.contains("Camera") => {
                self.cameras.remove(&entity);
            }
            _ if type_name.contains("PointLight") => {
                self.point_lights.remove(&entity);
            }
            _ if type_name.contains("DirectionalLight") => {
                self.directional_lights.remove(&entity);
            }
            _ if type_name.contains("MeshPrimitive") => {
                self.mesh_primitives.remove(&entity);
            }
            _ if type_name.contains("LightToggle") => {
                self.light_toggles.remove(&entity);
            }
            _ => {}
        }

        if let Some(components) = self.entities.get_mut(&entity) {
            components.retain(|c| c != type_name);
            true
        } else {
            false
        }
    }

    pub fn entity_has_components(&self, entity: Entity, component_types: &[&str]) -> bool {
        if let Some(components) = self.entities.get(&entity) {
            component_types.iter().all(|t| components.iter().any(|c| c.contains(*t)))
        } else {
            false
        }
    }

    pub fn iter_entities_with_components(&self, component_types: &[&str]) -> Vec<Entity> {
        self.entities
            .keys()
            .filter(|e| self.entity_has_components(**e, component_types))
            .copied()
            .collect()
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_creation() {
        let mut world = World::new();
        let e1 = world.create_entity();
        let e2 = world.create_entity();
        
        assert_eq!(e1.id, 0);
        assert_eq!(e2.id, 1);
        assert_ne!(e1, e2);
    }

    #[test]
    fn test_component_insertion_retrieval() {
        let mut world = World::new();
        let entity = world.create_entity();
        
        let transform = Transform::identity();
        world.insert_component(entity, transform.clone());
        
        let retrieved = world.get_component::<Transform>(entity);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().position, Vec3::ZERO);
    }

    #[test]
    fn test_camera_component() {
        let mut world = World::new();
        let entity = world.create_entity();
        
        let camera = Camera::perspective(1.0, 16.0/9.0, 0.1, 100.0);
        world.insert_component(entity, camera);
        
        assert!(world.get_component::<Camera>(entity).is_some());
    }
}
