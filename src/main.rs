use bevy::prelude::*;
use bevy::app::AppExit;
use std::f32::consts::TAU;
use bevy::input::mouse::MouseWheel;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(debug_controls)
        .add_system(update_scene)
        .add_startup_system(load_scene)
        .run();
}

fn load_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(SceneBundle {
        scene: asset_server.load("test.glb#Scene0"),
        ..default()
    });

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.50,
    });

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-20.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })
    .insert(PanOrbitCamera::default());
}

fn update_scene(
    windows: Res<Windows>,
    mut ev_scroll: EventReader<MouseWheel>,
    mut cameras: Query<(&mut PanOrbitCamera, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut pan_orbit, mut transform) in &mut cameras {
        let mut rotation_move = Vec2::ZERO;
        let mut scroll = 0.0;

        for ev in ev_scroll.iter() {
            scroll += ev.y;
        }

        rotation_move.x += 120.0 * time.delta_seconds();

        if scroll.abs() > 0.0 {
            pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
            pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
        }

        if rotation_move.length_squared() > 0.0 {
            let window = get_primary_window_size(&windows);
            let delta_x = {
                let delta = rotation_move.x / window.x * TAU;
                if pan_orbit.upside_down {
                    -delta
                } else {
                    delta
                }
            };
            let delta_y = rotation_move.y / window.y * (TAU * 2.0);
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation; // rotate around global y axis
            transform.rotation *= pitch; // rotate around local x axis

            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation =
                pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
        }
    }
}

fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    Vec2::new(window.width() as f32, window.height() as f32)
}

fn debug_controls(
    keys: Res<Input<KeyCode>>,
    mut exit: ResMut<Events<AppExit>>,
) {
    if keys.just_pressed(KeyCode::Q) {
        exit.send(AppExit);
    }
}

#[derive(Component)]
pub struct PanOrbitCamera {
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 55.0,
            upside_down: false,
        }
    }
}
