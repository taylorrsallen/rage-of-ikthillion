use crate::*;

use bevy::{
    app::prelude::*,
    ecs::{bundle::Bundle, prelude::*},
    input::{mouse::MouseMotion, prelude::*},
    math::prelude::*,
    time::Time,
    transform::components::Transform,
};

//================================-================================-================================
pub struct FpsCameraPlugin;
impl Plugin for FpsCameraPlugin {
    fn build(
        &self,
        app: &mut App
    ) {
        app.add_system(default_input_map)
        .add_system(control_system)
        .add_event::<ControlEvent>();
    }
}

//================================-================================-================================
#[derive(Bundle)]
pub struct FpsCameraBundle {
    controller: FpsCameraController,
    #[bundle]
    look_transform: LookTransform,
    transform: Transform,
}

impl FpsCameraBundle {
    pub fn new(controller: FpsCameraController, eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let transform = Transform::from_translation(eye).looking_at(target, up);

        Self {
            controller,
            look_transform: LookTransform::new(eye, target, up),
            transform,
        }
    }
}

#[derive(Default, Clone, Component, Copy, Debug)]
pub struct FpsCameraController;

pub enum ControlEvent {
    Rotate(Vec2),
}

pub fn default_input_map(
    mut events: EventWriter<ControlEvent>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    window_query: Query<&Window>,
    settings: Res<Settings>,
) {
    let window = window_query.single();
    if !window.cursor.visible {
        let mut cursor_delta = Vec2::ZERO;
        for event in mouse_motion_events.iter() {
            cursor_delta += event.delta;
        }
    
        events.send(ControlEvent::Rotate(
            settings.look_sensitivity * cursor_delta,
        ));
    }
}

pub fn control_system(
    mut events: EventReader<ControlEvent>,
    mut camera_query: Query<(&mut Transform, &mut LookTransform), With<FpsCameraController>>,
    time: Res<Time>,
) {
    for (mut transform, mut look_transform) in camera_query.iter_mut() {
        let look_vector = look_transform.look_direction().unwrap();
        let mut look_angles = LookAngles::from_vector(look_vector);

        let yaw_rot = Quat::from_axis_angle(Vec3::Y, look_angles.get_yaw());
        let rot_x = yaw_rot * Vec3::X;
        let rot_y = yaw_rot * Vec3::Y;
        let rot_z = yaw_rot * Vec3::Z;

        let dt = time.delta_seconds();
        for event in events.iter() {
            match event {
                ControlEvent::Rotate(delta) => {
                    look_angles.add_yaw(dt * -delta.x);
                    look_angles.add_pitch(dt * -delta.y);
                }
            }
        }

        look_transform.target = look_transform.eye + look_transform.radius() * look_angles.unit_vector();
        *transform = Transform::from(*look_transform);
    }
}