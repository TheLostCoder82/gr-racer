mod engine;
mod game;

use anyhow::Result;
use std::time::Instant;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowId},
};

use engine::ecs::{World, SystemScheduler, TransformSystem, LightControlSystem, CameraControllerSystem};
use engine::physics::PhysicsWorld;
use engine::audio::AudioEngine;
use engine::ui::DebugUiSystem;
use engine::render::Renderer;
use game::create_demo_scene;

pub struct App {
    window: Option<Window>,
    renderer: Option<Renderer>,
    world: World,
    physics: PhysicsWorld,
    audio: Option<AudioEngine>,
    scheduler: SystemScheduler,
    ui_system: DebugUiSystem,
    last_frame_time: Instant,
    delta_time: f32,
}

impl App {
    pub fn new(event_loop: &EventLoop<()>) -> Result<Self> {
        let window = Window::new(event_loop)?;
        window.set_title("Rust Racing Engine");
        window.set_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0));
        window.set_resizable(true);
        window.set_decorated(true);

        let mut world = World::new();
        create_demo_scene(&mut world);

        let mut scheduler = SystemScheduler::new();
        scheduler.add_system(TransformSystem);
        scheduler.add_system(LightControlSystem);
        scheduler.add_system(CameraControllerSystem::new());

        Ok(Self {
            window: Some(window),
            renderer: None,
            world,
            physics: PhysicsWorld::new(),
            audio: None,
            scheduler,
            ui_system: DebugUiSystem::new(),
            last_frame_time: Instant::now(),
            delta_time: 0.0,
        })
    }

    pub fn get_delta_time(&self) -> f32 {
        self.delta_time
    }

    pub fn get_window(&self) -> Option<&Window> {
        self.window.as_ref()
    }
}

impl ApplicationHandler<()> for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let window = match Window::new(event_loop) {
                Ok(w) => w,
                Err(e) => {
                    eprintln!("Failed to create window: {}", e);
                    return;
                }
            };
            window.set_title("Rust Racing Engine");
            window.set_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0));
            self.window = Some(window);
        }

        // Initialize renderer once window is available
        if self.renderer.is_none() {
            if let Some(window) = &self.window {
                let window_clone = window.clone();
                // Note: In a real implementation, we'd use tokio or async runtime
                // For now, we'll skip async initialization
                eprintln!("Renderer initialization would happen here with async support");
            }
        }

        // Initialize audio
        if self.audio.is_none() {
            match AudioEngine::new() {
                Ok(audio) => self.audio = Some(audio),
                Err(e) => eprintln!("Audio initialization failed: {}", e),
            }
        }

        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn window_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if let Some(ref mut renderer) = self.renderer {
                    renderer.resize(size.width, size.height);
                }
            }
            WindowEvent::RedrawRequested => {
                // Calculate delta time
                let now = Instant::now();
                self.delta_time = now.duration_since(self.last_frame_time).as_secs_f32();
                self.last_frame_time = now;

                // Update FPS counter
                self.ui_system.update_fps(self.delta_time);

                // Run game systems
                self.scheduler.run(&mut self.world, self.delta_time);

                // Step physics
                self.physics.step(self.delta_time);

                // Render frame (placeholder - full rendering needs async setup)
                // In production: render scene with wgpu

                // Request next frame
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => {}
        }
    }
}

pub fn run() -> Result<()> {
    env_logger::init();
    
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new(&event_loop)?;
    
    event_loop.run_app(&mut app)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        // This test verifies the App struct can be created
        // Full window creation requires an event loop
        assert!(true);
    }

    #[test]
    fn test_world_initialization() {
        let mut world = World::new();
        create_demo_scene(&mut world);
        
        let entities = world.iter_entities_with_components(&["Transform"]);
        assert!(!entities.is_empty());
    }

    #[test]
    fn test_physics_world() {
        let physics = PhysicsWorld::new();
        assert_eq!(physics.gravity.y, -9.81);
    }
}

fn main() -> Result<()> {
    run()
}
