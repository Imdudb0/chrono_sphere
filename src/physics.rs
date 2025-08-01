use bevy::prelude::*;
use crate::sphere::{Sphere, SphereVelocity};
use crate::chrono_slowmo::ChronoState;

#[derive(Resource)]
pub struct PhysicsSettings {
    pub gravity: Vec3,
    pub air_resistance: f32,
    pub bounce_factor: f32,
}

impl Default for PhysicsSettings {
    fn default() -> Self {
        Self {
            gravity: Vec3::new(0.0, -9.81, 0.0),
            air_resistance: 0.02,
            bounce_factor: 0.7,
        }
    }
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PhysicsSettings>()
            .add_systems(Update, (
                apply_gravity,
                apply_air_resistance,
                handle_bounds_collision,
            ).chain());
    }
}

fn apply_gravity(
    time: Res<Time>,
    physics: Res<PhysicsSettings>,
    chrono_state: Res<ChronoState>,
    mut sphere_query: Query<&mut SphereVelocity, With<Sphere>>,
) {
    let delta = time.delta_seconds() * chrono_state.current_time_scale;
    for mut velocity in sphere_query.iter_mut() {
        velocity.velocity += physics.gravity * delta;
    }
}

fn apply_air_resistance(
    time: Res<Time>,
    physics: Res<PhysicsSettings>,
    chrono_state: Res<ChronoState>,
    mut sphere_query: Query<&mut SphereVelocity, With<Sphere>>,
) {
    let resistance_factor = 1.0 - (physics.air_resistance * time.delta_seconds() * chrono_state.current_time_scale);
    for mut velocity in sphere_query.iter_mut() {
        velocity.velocity *= resistance_factor;
    }
}

fn handle_bounds_collision(
    physics: Res<PhysicsSettings>,
    mut sphere_query: Query<(&mut Transform, &mut SphereVelocity, &Sphere), With<Sphere>>,
) {
    for (mut transform, mut velocity, sphere) in sphere_query.iter_mut() {
        let position = transform.translation;
        let radius = sphere.radius;
        
        // Collision avec le sol
        if position.y - radius <= -5.0 {
            transform.translation.y = -5.0 + radius;
            velocity.velocity.y = -velocity.velocity.y * physics.bounce_factor;
            
            // Réduction de l'énergie horizontale lors de l'impact
            velocity.velocity.x *= 0.9;
            velocity.velocity.z *= 0.9;
        }
        
        // Limites latérales (environnement de test)
        let bounds = 25.0;
        if position.x.abs() > bounds {
            transform.translation.x = bounds * position.x.signum();
            velocity.velocity.x = -velocity.velocity.x * physics.bounce_factor;
        }
        
        if position.z.abs() > bounds {
            transform.translation.z = bounds * position.z.signum();
            velocity.velocity.z = -velocity.velocity.z * physics.bounce_factor;
        }
    }
}
