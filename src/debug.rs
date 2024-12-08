use crate::prelude::*;

#[derive(Debug)]
pub enum DebugConfigKey {
    LightIntensityIncrease,
    LightIntensityDecrease,
    LightHeightIncrease,
    LightHeightDecrease,
}

#[derive(Event, Debug)]
pub struct UpdateDebugConfigEvent {
    pub config_key: DebugConfigKey,
}

#[derive(Resource, Debug)]
pub struct DebugConfig {
    pub light_height: DebugConfigValue,
    pub light_intensity: DebugConfigValue,
}

#[derive(Debug)]
pub struct DebugConfigValue {
    pub value: i32,
    pub min: i32,
    pub max: i32,
}

pub fn update_debug_config_system(
    mut update_debug_config_event: EventReader<UpdateDebugConfigEvent>,
    mut debug_config: ResMut<DebugConfig>,
) {
    for event in update_debug_config_event.read() {
        match event.config_key {
            DebugConfigKey::LightIntensityIncrease => {
                debug_config.light_intensity.value =
                    (debug_config.light_intensity.value + 1).min(debug_config.light_intensity.max);
            }
            DebugConfigKey::LightIntensityDecrease => {
                debug_config.light_intensity.value =
                    (debug_config.light_intensity.value - 1).max(debug_config.light_intensity.min);
            }
            DebugConfigKey::LightHeightIncrease => {
                debug_config.light_height.value =
                    (debug_config.light_height.value + 1).min(debug_config.light_height.max);
            }
            DebugConfigKey::LightHeightDecrease => {
                debug_config.light_height.value =
                    (debug_config.light_height.value - 1).max(debug_config.light_height.min);
            }
        }
    }
}
