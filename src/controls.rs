use bevy::prelude::*;
use bevy::input::touch::*;
use crate::sphere::{Sphere, SphereVelocity};

#[derive(Resource)]
pub struct ControlSettings {
    pub tilt_sensitivity: f32,
    pub touch_force_multiplier: f32,
    pub max_tilt_force: f32,
}

impl Default for ControlSettings {
    fn default() -> Self {
        Self {
            tilt_sensitivity: 15.0,
            touch_force_multiplier: 8.0,
            max_tilt_force: 20.0,
        }
    }
}

#[derive(Resource, Default)]
pub struct InputState {
    pub tilt_input: Vec2,
    pub is_touching: bool,
    pub touch_position: Vec2,
}

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ControlSettings>()
            .init_resource::<InputState>()
            .add_systems(Update, (
                handle_keyboard_input,     // Pour les tests sur desktop
                handle_touch_input,        // Pour mobile
                apply_tilt_controls,
                apply_touch_force,
            ).chain());
    }
}

// Simulation d'inclinaison via clavier pour les tests
fn handle_keyboard_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut input_state: ResMut<InputState>,
) {
    let mut tilt = Vec2::ZERO;
    
    if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        tilt.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        tilt.x += 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
        tilt.y += 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
        tilt.y -= 1.0;
    }
    
    input_state.tilt_input = tilt;
    input_state.is_touching = keyboard.pressed(KeyCode::Space);
}

fn handle_touch_input(
    touches: Res<Touches>,
    mut input_state: ResMut<InputState>,
    windows: Query<&Window>,
) {
    if let Ok(window) = windows.get_single() {
        if let Some(touch) = touches.iter().next() {
            input_state.is_touching = true;
            input_state.touch_position = touch.position();
            
            // Convertir la position tactile en inclinaison simulée
            let center = Vec2::new(window.width() / 2.0, window.height() / 2.0);
            let touch_offset = (touch.position() - center) / center;
            
            // Limiter et ajuster la sensibilité
            input_state.tilt_input = touch_offset.clamp_length_max(1.0);
        } else {
            input_state.is_touching = false;
            // Retour progressif au centre quand pas de toucher
            input_state.tilt_input *= 0.9;
        }
    }
}

fn apply_tilt_controls(
    time: Res<Time>,
    control_settings: Res<ControlSettings>,
    input_state: Res<InputState>,
    chrono_state: Res<crate::chrono_slowmo::ChronoState>,
    mut sphere_query: Query<&mut SphereVelocity, With<Sphere>>,
) {
    let delta = time.delta_seconds() * chrono_state.current_time_scale;
    for mut velocity in sphere_query.iter_mut() {
        let tilt_force = Vec3::new(
            input_state.tilt_input.x * control_settings.tilt_sensitivity,
            0.0,
            -input_state.tilt_input.y * control_settings.tilt_sensitivity,
        );
        
        // Appliquer la force avec limitation
        let clamped_force = tilt_force.clamp_length_max(control_settings.max_tilt_force);
        velocity.velocity += clamped_force * delta;
    }
}

fn apply_touch_force(
    time: Res<Time>,
    control_settings: Res<ControlSettings>,
    input_state: Res<InputState>,
    chrono_state: Res<crate::chrono_slowmo::ChronoState>,
    mut sphere_query: Query<&mut SphereVelocity, With<Sphere>>,
) {
    if input_state.is_touching {
        let delta = time.delta_seconds() * chrono_state.current_time_scale;
        for mut velocity in sphere_query.iter_mut() {
            // Force supplémentaire quand on touche l'écran
            let touch_boost = Vec3::new(
                input_state.tilt_input.x * control_settings.touch_force_multiplier,
                2.0, // Petit boost vertical
                -input_state.tilt_input.y * control_settings.touch_force_multiplier,
            );
            
            velocity.velocity += touch_boost * delta;
        }
    }
}
