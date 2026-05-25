**Response Structure Followed Strictly**

---

### 1. Phase-by-Phase Breakdown

#### Phase 1: Project Setup & Crate Configuration
**Task 1.1: Initialize Cargo Project and Add Dependencies**
* **Description:** Create a new Rust binary project and configure `Cargo.toml` with exact versions of required crates: `winit`, `wgpu`, `glam`, `rapier3d`, `egui`, `egui-wgpu`, `egui-winit`, `kira`, and necessary features.
* **Implementation GitHub Copilot Prompt:** "Create a new Rust project Cargo.toml for a 3D game engine. Include: winit = { version = '0.30', features = ['rwh_05'] }, wgpu = { version = '24', features = ['webgl'] }, glam = { version = '0.29', features = ['bytemuck'] }, rapier3d = { version = '0.19', features = ['serde', 'parallel'] }, egui = '0.31', egui-winit = '0.31', egui-wgpu = '0.31', kira = { version = '0.9', features = ['ogg', 'flac', 'wav'] }. Add anyhow, env_logger, and pollster. Set edition 2021 and optimize for performance."
* **Validation GitHub Copilot Prompt:** "Write a unit test that parses Cargo.toml and verifies all required crates and their features are present with correct versions."

**Task 1.2: Create Project Structure**
* **Description:** Set up standard folders: `src/engine/core`, `src/engine/render`, `src/engine/ecs`, `src/engine/physics`, `src/engine/audio`, `src/engine/ui`, `src/game`, `assets/models`, `assets/sounds`.
* **Implementation GitHub Copilot Prompt:** "Create the complete folder structure and empty mod.rs files for a Rust game engine: engine/render, engine/ecs, engine/physics, engine/audio, engine/ui, and game modules."
* **Validation GitHub Copilot Prompt:** "Write tests that assert all required directories and mod.rs files exist in the project structure."

#### Phase 2: Window & Event Loop (winit)
**Task 2.1: Create Application Window**
* **Description:** Initialize winit event loop and create a window with proper size (1280x720) and title "Rust Racing Engine".
* **Implementation GitHub Copilot Prompt:** "Create a struct App with winit::event_loop::EventLoop and winit::window::Window. Implement new() that creates a window 1280x720 titled 'Rust Racing Engine' with decorations and resizable. Include proper error handling with anyhow."
* **Validation GitHub Copilot Prompt:** "Generate unit tests for App::new() that verify window dimensions, title, and that the event loop is properly initialized."

**Task 2.2: Implement Main Event Loop**
* **Description:** Set up the main run loop handling WindowEvent, DeviceEvent, and MainEventsCleared for game loop timing.
* **Implementation GitHub Copilot Prompt:** "Implement the run() method for the App struct using winit event loop. Handle RedrawRequested, WindowEvent::CloseRequested, and calculate delta time using Instant. Use pollster for async wgpu initialization later."
* **Validation GitHub Copilot Prompt:** "Write integration tests using winit test helpers to verify event loop processes CloseRequested and RedrawRequested correctly."

#### Phase 3: WGPU Rendering Engine Setup (Modern + GL Fallback)
**Task 3.1: Initialize WGPU Instance & Adapter**
* **Description:** Create wgpu::Instance with WebGL fallback support and request a high-performance adapter.
* **Implementation GitHub Copilot Prompt:** "Create a Renderer struct. In new(), initialize wgpu::Instance with backends::all() and features for WebGL fallback. Request adapter with power_preference HighPerformance and fallback to LowPower."
* **Validation GitHub Copilot Prompt:** "Generate tests that verify wgpu Instance is created with proper backends and adapter supports WebGL fallback."

**Task 3.2: Create Device, Queue, and Surface**
* **Description:** Configure surface, device, and queue with required features for 3D rendering.
* **Implementation GitHub Copilot Prompt:** "Extend Renderer to create surface from window, logical device with features TEXTURE_COMPRESSION_BC + SHADER_F16, and queue. Configure surface with proper format (Rgba8UnormSrgb or Bgra8UnormSrgb) and PresentMode::Fifo."
* **Validation GitHub Copilot Prompt:** "Write tests confirming device limits, surface configuration, and preferred format selection."

**Task 3.3: Implement Basic Render Pipeline**
* **Description:** Create shader modules, pipeline layout, and render pipeline for colored 3D geometry.
* **Implementation GitHub Copilot Prompt:** "Create vertex and fragment shaders for basic 3D rendering with position + color. Build RenderPipeline with depth/stencil state, multisampling off, and primitive topology TriangleList. Support wireframe toggle."
* **Validation GitHub Copilot Prompt:** "Generate tests that compile shaders and validate pipeline creation with correct bind group layouts."

#### Phase 4: Custom ECS Implementation
**Task 4.1: Define ECS Core (World, Entity, Component)**
* **Description:** Build a simple custom ECS using generational-arena or Vec storage for Entities and HashMap-based component storage.
* **Implementation GitHub Copilot Prompt:** "Implement a basic ECS: Entity(u64), World struct with generational index allocator, and Component trait. Support insert_component<T: Component> and get_component<T>()."
* **Validation GitHub Copilot Prompt:** "Write comprehensive unit tests for entity creation, component insertion, retrieval, and removal with proper generational indexing."

**Task 4.2: Implement System Trait and Scheduler**
* **Description:** Define System trait with update(&mut self, world: &mut World, dt: f32) and a simple scheduler that runs systems in order.
* **Implementation GitHub Copilot Prompt:** "Create System trait and SystemScheduler that holds Vec<Box<dyn System>>. Implement run() that calls update on each system with delta time."
* **Validation GitHub Copilot Prompt:** "Create tests that verify systems execute in registration order and receive correct delta time."

#### Phase 5: Base Components & Core Systems
**Task 5.1: Transform, Camera, Light, and Primitive Components**
* **Description:** Define components: Transform (position, rotation, scale using glam), Camera, PointLight/DirectionalLight, Mesh (primitive type: Cube, Sphere, etc.).
* **Implementation GitHub Copilot Prompt:** "Define Rust structs for Transform, Camera (view/projection matrices), PointLight (position, color, intensity), and MeshPrimitive enum (Cube, Sphere). All deriving Component trait."
* **Validation GitHub Copilot Prompt:** "Write tests ensuring components can be inserted into ECS and their data is correctly stored and retrieved."

**Task 5.2: Transform System & Light Toggle System**
* **Description:** Implement systems for updating transforms and toggling lights on/off.
* **Implementation GitHub Copilot Prompt:** "Create TransformSystem that applies parent-child transforms if needed. Create LightControlSystem that processes LightToggle components to enable/disable lights."
* **Validation GitHub Copilot Prompt:** "Generate unit tests verifying TransformSystem updates matrices correctly and LightControlSystem toggles visibility/intensity."

#### Phase 6: Physics Integration (Rapier)
**Task 6.1: Physics World Setup**
* **Description:** Integrate Rapier3D physics world with gravity.
* **Implementation GitHub Copilot Prompt:** "Create PhysicsSystem that manages rapier3d::physics::RigidBodySet, ColliderSet, and IntegrationParameters. Set gravity to (0.0, -9.81, 0.0)."
* **Validation GitHub Copilot Prompt:** "Test physics world initialization with correct gravity vector."

**Task 6.2: Physics Sync System**
* **Description:** Sync Transform components with Rapier rigid bodies.
* **Implementation GitHub Copilot Prompt:** "Implement PhysicsSyncSystem that updates Transform from rigid body positions and vice versa for dynamic objects."
* **Validation GitHub Copilot Prompt:** "Write tests that verify position synchronization between ECS Transform and Rapier rigid bodies."

#### Phase 7: Audio System (Kira)
**Task 7.1: Audio Manager Setup**
* **Description:** Initialize Kira audio manager with spatial support.
* **Implementation GitHub Copilot Prompt:** "Create AudioManager struct wrapping kira::Manager. Load sound effects for collision (impact_low, impact_medium)."
* **Validation GitHub Copilot Prompt:** "Test audio manager initialization and sound loading from assets/sounds/."

**Task 7.2: Collision Sound System**
* **Description:** Play sound on physics collision events.
* **Implementation GitHub Copilot Prompt:** "Create CollisionSoundSystem that listens to Rapier contact events and plays appropriate Kira sound effect based on impact velocity."
* **Validation GitHub Copilot Prompt:** "Write mock tests simulating collisions and verifying sound playback is triggered with correct parameters."

#### Phase 8: UI Integration (egui)
**Task 8.1: Egui Renderer Setup**
* **Description:** Integrate egui with wgpu renderer.
* **Implementation GitHub Copilot Prompt:** "Add EguiRenderer to main Renderer. Implement egui-wgpu renderer integration with full output handling."
* **Validation GitHub Copilot Prompt:** "Test egui context creation and texture integration with wgpu."

**Task 8.2: FPS Counter & Light Toggle UI**
* **Description:** Build UI panel showing FPS and toggle for main light.
* **Implementation GitHub Copilot Prompt:** "Create DebugUiSystem that renders egui window with FPS counter (using smoothed moving average) and checkbox to toggle primary light entity."
* **Validation GitHub Copilot Prompt:** "Generate tests verifying UI state updates correctly affect ECS light component."

#### Phase 9: Example Racing Game Scene
**Task 9.1: Scene Assembly**
* **Description:** Create a demo scene with ground plane, several dynamic primitives (cubes/spheres), one directional light, and a movable camera.
* **Implementation GitHub Copilot Prompt:** "In game::demo_scene.rs, create function that spawns ground (static rigid body), 8 dynamic primitives with random positions, one PointLight, and Camera entity."
* **Validation GitHub Copilot Prompt:** "Test scene creation by counting entities and verifying presence of required components."

**Task 9.2: Camera Controller**
* **Description:** Implement WASD + mouse look camera movement.
* **Implementation GitHub Copilot Prompt:** "Create CameraController system that processes keyboard input (WASD) and mouse delta for first-person style movement and rotation."
* **Validation GitHub Copilot Prompt:** "Write input simulation tests for camera movement in all directions."

**Task 9.3: Main Game Loop Integration**
* **Description:** Wire up all systems in the correct order in the main update loop: Input → Physics → Transform Sync → Render → Audio → UI.
* **Implementation GitHub Copilot Prompt:** "In main App update loop, register and run systems in order: Input, Physics, Sync, RenderPrep, Audio, UI."
* **Validation GitHub Copilot Prompt:** "Verify full system execution order and integration through integration test with mocked delta times."

---

### 2. Phase-Gate Validation Prompts

**Phase 1 Gate:**
"Review all files in Cargo.toml and src/ directory. Verify all dependencies are correctly declared with features. Confirm project structure matches specification. Check for any missing modules or compilation errors. Output 'PHASE 1 COMPLETE' only if everything is correct."

**Phase 2 Gate:**
"Analyze src/main.rs and window-related files. Verify winit event loop is robust, handles all critical events, and delta time is calculated accurately. Ensure no blocking calls. Confirm 'PHASE 2 COMPLETE'."

**Phase 3 Gate:**
"Examine all wgpu-related code. Verify instance, adapter, device, surface, and render pipeline are correctly configured with GL fallback. Check shader compilation. Confirm modern rendering pipeline is ready. Output 'PHASE 3 COMPLETE'."

**Phase 4-9 Gates:** (Similar pattern for each phase)
"Review all files modified in Phase X. Verify ECS systems, components, physics, audio, UI integrate without circular dependencies or technical debt. Run cargo check and clippy. Confirm phase requirements are 100% met before allowing progression. Output 'PHASE X COMPLETE'."

---

### 3. Final Codebase Analysis & Verification Prompt

**Master Verification Prompt:**

```
You are an expert Rust game engine auditor.

Scan the entire codebase for the 3D Racing Game Engine built with winit, wgpu, glam, rapier3d, egui, and kira.

Requirements to verify:
1. Modern wgpu renderer with GL fallback support.
2. Fully functional custom ECS with Transform, Camera, Light, MeshPrimitive components.
3. Working systems: Transform, PhysicsSync, LightControl, CollisionSound, CameraController, DebugUI.
4. Rapier physics with gravity affecting primitives.
5. Kira audio with collision impact sounds.
6. Egui UI showing FPS counter and light toggle.
7. Demo scene with at least 1 light, movable camera, multiple 3D primitives demonstrating physics.

Perform:
- Full cargo check + clippy --all-targets -- -D warnings
- Cross-reference every implemented feature against the original spec
- Identify any security issues, memory leaks, unhandled edge cases (especially window resize, GPU loss)
- Verify architectural consistency and performance considerations
- Test mentally all critical paths: collision → sound, light toggle, camera movement, rendering

Provide a detailed report and a final "Definition of Done" checklist with Yes/No for each major requirement. Only declare "PRODUCTION-READY" if every item passes with no critical issues.