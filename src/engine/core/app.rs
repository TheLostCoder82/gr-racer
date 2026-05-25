use anyhow::Result;
use std::time::Instant;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowId},
};

pub struct App {
    window: Option<Window>,
    last_frame_time: Instant,
    delta_time: f32,
}

impl App {
    pub fn new(event_loop: &EventLoop<()>) -> Result<Self> {
        let window = Window::new(event_loop).map_err(|e| anyhow::anyhow!("Failed to create window: {}", e))?;
        window.set_title("Rust Racing Engine");
        window.set_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0));
        window.set_resizable(true);
        window.set_decorated(true);

        Ok(Self {
            window: Some(window),
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
    }

    fn window_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // Calculate delta time
                let now = Instant::now();
                self.delta_time = now.duration_since(self.last_frame_time).as_secs_f32();
                self.last_frame_time = now;

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
    
    // Simple event loop using run_app
    event_loop.run_app(&mut app)?;

    Ok(())
}
