use bevy::input::mouse::{MouseButtonInput, MouseMotion};
use bevy::input::ButtonState;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use std::f32::consts::{FRAC_PI_2, TAU};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_fps_camera, cursor_lock, cursor_unlock));
    }
}

#[derive(Debug, Default, Copy, Clone, Component)]
pub struct FpsCam {
    pub pitch: f32,
    pub yaw: f32,
}

const MOVE_SPEED: f32 = 3.0;
const PAN_SPEED: Vec2 = vec2(0.1, 0.1);

fn handle_fps_camera(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut motion: EventReader<MouseMotion>,
    mut q_camera: Query<(&mut Transform, &mut FpsCam), With<FpsCam>>,
) {
    let mouse_delta =
        motion.read().fold(Vec2::ZERO, |acc, ev| acc + ev.delta) * PAN_SPEED * time.delta_seconds();
    let window = q_windows.single();
    if window.cursor.grab_mode == CursorGrabMode::None {
        return;
    }

    for (mut transform, mut fps_cam) in &mut q_camera {
        let mut translation = Vec3::ZERO;

        let forward: Vec3 = transform.forward().into();
        let right: Vec3 = transform.right().into();

        if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
            translation += forward;
        }
        if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
            translation -= forward;
        }
        if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
            translation -= right;
        }
        if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
            translation += right;
        }

        translation = translation.normalize_or_zero();
        transform.translation += translation * time.delta_seconds() * MOVE_SPEED;

        fps_cam.yaw -= mouse_delta.x;
        if fps_cam.yaw > TAU {
            fps_cam.yaw -= TAU;
        }
        fps_cam.pitch -= mouse_delta.y;
        fps_cam.pitch = fps_cam.pitch.clamp(-FRAC_PI_2, FRAC_PI_2);

        transform.rotation = Quat::from_axis_angle(Vec3::Y, fps_cam.yaw)
            * Quat::from_axis_angle(Vec3::X, fps_cam.pitch);
    }
}

fn cursor_lock(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    mut mouse_events: EventReader<MouseButtonInput>,
) {
    let mut window = q_windows.single_mut();
    let is_pressed = mouse_events
        .read()
        .any(|ev| ev.state == ButtonState::Pressed);
    if is_pressed {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    }
}

fn cursor_unlock(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let mut window = q_windows.single_mut();
    if keys.just_released(KeyCode::Escape) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
}
