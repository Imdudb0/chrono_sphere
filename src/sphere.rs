use bevy::prelude::*;

#[derive(Component)]
pub struct Sphere {
    pub radius: f32,
    pub energy_level: f32,
}

#[derive(Component)]
pub struct SphereVelocity {
    pub velocity: Vec3,
    pub max_speed: f32,
}

pub struct SpherePlugin;

impl Plugin for SpherePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_sphere)
            .add_systems(Update, (
                update_sphere_visual,
                apply_sphere_momentum,
            ));
    }
}

fn spawn_sphere(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Configuration de la caméra
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 3.0, 8.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Lumière principale
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 10.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });

    // La sphère énergétique avec API moderne Bevy 0.16
    let sphere_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.2, 0.8, 1.0, 0.9),
        emissive: LinearRgba::rgb(0.1, 0.4, 0.6),
        metallic: 0.8,
        perceptual_roughness: 0.1,
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(1.0).mesh().ico(5).unwrap()),
            material: sphere_material,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Sphere {
            radius: 1.0,
            energy_level: 1.0,
        },
        SphereVelocity {
            velocity: Vec3::ZERO,
            max_speed: 10.0,
        },
        Name::new("ChronoSphere"),
    ));

    // Sol de référence temporaire avec API moderne
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.1, 0.15),
            metallic: 0.0,
            perceptual_roughness: 0.8,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, -5.0, 0.0),
        ..default()
    });
}

fn update_sphere_visual(
    time: Res<Time>,
    mut sphere_query: Query<(&mut Transform, &Sphere), With<Sphere>>,
) {
    for (mut transform, sphere) in sphere_query.iter_mut() {
        // Effet de pulsation énergétique
        let pulse = (time.elapsed_seconds() * 3.0).sin() * 0.1 + 1.0;
        let energy_scale = sphere.energy_level * pulse;
        
        transform.scale = Vec3::splat(energy_scale);
        
        // Rotation légère pour l'effet visuel
        transform.rotate_y(time.delta_seconds() * 0.5);
    }
}

fn apply_sphere_momentum(
    time: Res<Time>,
    chrono_state: Res<crate::chrono_slowmo::ChronoState>,
    mut sphere_query: Query<(&mut Transform, &mut SphereVelocity), With<Sphere>>,
) {
    let delta = time.delta_seconds() * chrono_state.current_time_scale;
    for (mut transform, mut velocity) in sphere_query.iter_mut() {
        // Application de la vélocité
        transform.translation += velocity.velocity * delta;
        
        // Friction basique
        velocity.velocity *= 0.98;
        
        // Limites de vitesse
        if velocity.velocity.length() > velocity.max_speed {
            velocity.velocity = velocity.velocity.normalize() * velocity.max_speed;
        }
    }
}
