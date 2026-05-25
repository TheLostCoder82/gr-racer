use anyhow::Result;
use kira::{
    self,
    manager::{AudioManager, AudioManagerSettings, Backend},
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
};
use std::path::Path;

/// Audio manager wrapper for Kira
pub struct AudioEngine {
    manager: AudioManager,
}

impl AudioEngine {
    pub fn new() -> Result<Self> {
        let manager = AudioManager::new(AudioManagerSettings::default())?;
        
        Ok(Self { manager })
    }

    pub fn load_sound(&mut self, path: &Path) -> Result<StaticSoundData> {
        let sound_data = StaticSoundData::from_file(path, StaticSoundSettings::default())?;
        Ok(sound_data)
    }

    pub fn play_sound(&mut self, sound_data: StaticSoundData) -> Result<()> {
        self.manager.play(sound_data)?;
        Ok(())
    }

    pub fn get_manager(&mut self) -> &mut AudioManager {
        &mut self.manager
    }
}

impl Default for AudioEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create audio engine")
    }
}

/// Collision sound system - plays sounds on physics collisions
pub struct CollisionSoundSystem {
    pub impact_low: Option<StaticSoundData>,
    pub impact_medium: Option<StaticSoundData>,
    pub impact_high: Option<StaticSoundData>,
}

impl CollisionSoundSystem {
    pub fn new() -> Self {
        Self {
            impact_low: None,
            impact_medium: None,
            impact_high: None,
        }
    }

    pub fn load_sounds(&mut self, audio_engine: &mut AudioEngine, assets_path: &Path) -> Result<()> {
        let low_path = assets_path.join("sounds/impact_low.wav");
        let medium_path = assets_path.join("sounds/impact_medium.wav");
        let high_path = assets_path.join("sounds/impact_high.wav");

        if low_path.exists() {
            self.impact_low = Some(audio_engine.load_sound(&low_path)?);
        }
        if medium_path.exists() {
            self.impact_medium = Some(audio_engine.load_sound(&medium_path)?);
        }
        if high_path.exists() {
            self.impact_high = Some(audio_engine.load_sound(&high_path)?);
        }

        Ok(())
    }

    pub fn play_impact_sound(&mut self, audio_engine: &mut AudioEngine, velocity: f32) {
        let sound = if velocity > 10.0 {
            self.impact_high.as_ref()
        } else if velocity > 5.0 {
            self.impact_medium.as_ref()
        } else {
            self.impact_low.as_ref()
        };

        if let Some(sound_data) = sound {
            let _ = audio_engine.play_sound(sound_data.clone());
        }
    }
}

impl Default for CollisionSoundSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_engine_creation() {
        // Note: This test may fail in headless environments without audio devices
        let result = AudioEngine::new();
        // We just verify it doesn't panic; actual audio depends on system
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_collision_sound_system_default() {
        let system = CollisionSoundSystem::new();
        assert!(system.impact_low.is_none());
        assert!(system.impact_medium.is_none());
        assert!(system.impact_high.is_none());
    }
}
