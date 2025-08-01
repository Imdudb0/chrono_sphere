use bevy::prelude::*;

mod sphere;
mod physics;
mod controls;
mod chrono_slowmo;

use sphere::SpherePlugin;
use physics::PhysicsPlugin;
use controls::ControlsPlugin;
use chrono_slowmo::ChronoSlowmoPlugin;

pub fn setup_chrono_sphere_app() -> App {
    let mut app = App::new();
    
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Chrono Sphere".to_string(),
            resolution: (800.0, 600.0).into(),
            ..default()
        }),
        ..default()
    }))
    .add_plugins((
        SpherePlugin,
        PhysicsPlugin,
        ControlsPlugin,
        ChronoSlowmoPlugin,
    ));
    
    app
}

#[cfg(target_os = "android")]
use bevy::winit::WinitPlugin;

#[bevy_main]
fn main() {
    setup_chrono_sphere_app().run();
}
