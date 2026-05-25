use egui::{Color32, FontId, Ui};

/// Debug UI system for FPS counter and light toggle
pub struct DebugUiSystem {
    pub fps: f64,
    pub fps_history: Vec<f64>,
    pub show_light_toggle: bool,
    pub light_enabled: bool,
}

impl DebugUiSystem {
    pub fn new() -> Self {
        Self {
            fps: 0.0,
            fps_history: Vec::with_capacity(60),
            show_light_toggle: true,
            light_enabled: true,
        }
    }

    pub fn update_fps(&mut self, delta_time: f32) {
        if delta_time > 0.0 {
            let current_fps = 1.0 / delta_time as f64;
            self.fps = current_fps;
            
            self.fps_history.push(current_fps);
            if self.fps_history.len() > 60 {
                self.fps_history.remove(0);
            }
        }
    }

    pub fn get_smoothed_fps(&self) -> f64 {
        if self.fps_history.is_empty() {
            return 0.0;
        }
        self.fps_history.iter().sum::<f64>() / self.fps_history.len() as f64
    }

    pub fn render(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("debug_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // FPS Counter
                let smoothed_fps = self.get_smoothed_fps();
                let fps_color = if smoothed_fps >= 55.0 {
                    Color32::GREEN
                } else if smoothed_fps >= 30.0 {
                    Color32::YELLOW
                } else {
                    Color32::RED
                };

                ui.label(
                    egui::RichText::new(format!("FPS: {:.1}", smoothed_fps))
                        .font(FontId::monospace(14.0))
                        .color(fps_color),
                );

                ui.separator();

                // Light toggle
                if self.show_light_toggle {
                    ui.checkbox(&mut self.light_enabled, "Main Light");
                }
            });
        });
    }

    pub fn is_light_enabled(&self) -> bool {
        self.light_enabled
    }

    pub fn toggle_light(&mut self) {
        self.light_enabled = !self.light_enabled;
    }
}

impl Default for DebugUiSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fps_calculation() {
        let mut ui_system = DebugUiSystem::new();
        
        // Simulate 60 FPS (delta time ~0.0167)
        ui_system.update_fps(0.0167);
        ui_system.update_fps(0.0167);
        ui_system.update_fps(0.0167);
        
        let smoothed = ui_system.get_smoothed_fps();
        assert!(smoothed > 50.0 && smoothed < 70.0);
    }

    #[test]
    fn test_light_toggle() {
        let mut ui_system = DebugUiSystem::new();
        assert!(ui_system.is_light_enabled());
        
        ui_system.toggle_light();
        assert!(!ui_system.is_light_enabled());
        
        ui_system.toggle_light();
        assert!(ui_system.is_light_enabled());
    }
}
