use bevy::prelude::*;
use crate::controls::InputState;

#[derive(Resource)]
pub struct ChronoSettings {
    pub slowmo_factor: f32,
    pub transition_speed: f32,
    pub energy_drain_rate: f32,
    pub energy_recharge_rate: f32,
}

impl Default for ChronoSettings {
    fn default() -> Self {
        Self {
            slowmo_factor: 0.3,
            transition_speed: 5.0,
            energy_drain_rate: 1.0,
            energy_recharge_rate: 0.5,
        }
    }
}

#[derive(Resource)]
pub struct ChronoState {
    pub current_time_scale: f32,
    pub target_time_scale: f32,
    pub chrono_energy: f32,
    pub is_slowmo_active: bool,
}

impl Default for ChronoState {
    fn default() -> Self {
        Self {
            current_time_scale: 1.0,
            target_time_scale: 1.0,
            chrono_energy: 100.0,
            is_slowmo_active: false,
        }
    }
}

pub struct ChronoSlowmoPlugin;

impl Plugin for ChronoSlowmoPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ChronoSettings>()
            .init_resource::<ChronoState>()
            .add_systems(Update, (
                handle_chrono_input,
                update_time_scale,
                manage_chrono_energy,
                apply_time_scale,
                update_visual_effects,
            ).chain());
    }
}

fn handle_chrono_input(
    input_state: Res<InputState>,
    mut chrono_state: ResMut<ChronoState>,
    chrono_settings: Res<ChronoSettings>,
) {
    let wants_slowmo = input_state.is_touching;
    let has_energy = chrono_state.chrono_energy > 0.0;
    
    if wants_slowmo && has_energy {
        chrono_state.is_slowmo_active = true;
        chrono_state.target_time_scale = chrono_settings.slowmo_factor;
    } else {
        chrono_state.is_slowmo_active = false;
        chrono_state.target_time_scale = 1.0;
    }
}

fn update_time_scale(
    time: Res<Time>,
    chrono_settings: Res<ChronoSettings>,
    mut chrono_state: ResMut<ChronoState>,
) {
    let transition_speed = chrono_settings.transition_speed;
    let delta = time.delta_seconds();
    
    // Transition fluide vers la vitesse cible
    let diff = chrono_state.target_time_scale - chrono_state.current_time_scale;
    chrono_state.current_time_scale += diff * transition_speed * delta;
    
    // Clamp pour éviter les dépassements
    chrono_state.current_time_scale = chrono_state.current_time_scale.clamp(0.1, 1.0);
}

fn manage_chrono_energy(
    time: Res<Time>,
    chrono_settings: Res<ChronoSettings>,
    mut chrono_state: ResMut<ChronoState>,
) {
    let delta = time.delta_seconds();
    
    if chrono_state.is_slowmo_active {
        // Drain de l'énergie pendant le slowmo
        chrono_state.chrono_energy -= chrono_settings.energy_drain_rate * delta * 100.0;
        chrono_state.chrono_energy = chrono_state.chrono_energy.max(0.0);
    } else {
        // Recharge de l'énergie
        chrono_state.chrono_energy += chrono_settings.energy_recharge_rate * delta * 100.0;
        chrono_state.chrono_energy = chrono_state.chrono_energy.min(100.0);
    }
}

fn apply_time_scale(
    chrono_state: Res<ChronoState>,
    mut time: ResMut<Time<Virtual>>,
) {
    // Application du facteur temporel à la physique
    let effective_scale = chrono_state.current_time_scale;
    time.set_relative_speed(effective_scale);
}

fn update_visual_effects(
    chrono_state: Res<ChronoState>,
    mut sphere_query: Query<&mut Handle<StandardMaterial>, With<crate::sphere::Sphere>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for material_handle in sphere_query.iter_mut() {
        if let Some(material) = materials.get_mut(&*material_handle) {
            let slowmo_intensity = 1.0 - chrono_state.current_time_scale;
            
            // Effet visuel : Plus le ralenti est actif, plus la sphère devient brillante
            let base_emissive = Color::rgb(0.1, 0.4, 0.6);
            let boosted_emissive = Color::rgb(
                0.1 + slowmo_intensity * 0.4,
                0.4 + slowmo_intensity * 0.4,
                0.6 + slowmo_intensity * 0.6,
            );
            
            material.emissive = boosted_emissive;
            
            // Effet de transparence pendant le slowmo
            let alpha = 0.9 + slowmo_intensity * 0.1;
            material.base_color = Color::rgba(0.2, 0.8, 1.0, alpha);
        }
    }
}
