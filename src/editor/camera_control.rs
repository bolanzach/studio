use bevy::{
    input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    prelude::*,
};

pub fn camera_control_system(
    mut query: Query<&mut Transform, With<Camera>>,
    mut scroll_evr: EventReader<MouseWheel>,
    mut mouse_evr: EventReader<MouseMotion>,
    key: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
) {
    let settings_sensitivity = 0.01;
    let mut rotation_move = Vec2::ZERO;
    let mut camera = query.get_single_mut().unwrap();

    // Zoom
    for event in scroll_evr.iter() {
        match event.unit {
            MouseScrollUnit::Line => println!("scroll LINE"),
            MouseScrollUnit::Pixel => {
                let new_z = camera.translation.z - event.y * settings_sensitivity;
                if new_z > 1.0 && new_z < 20.0 {
                    camera.translation.z = new_z;
                }
            }
        }
    }

    // Pan
    if mouse_input.pressed(MouseButton::Left) {
        for event in mouse_evr.iter() {
            let new_x = camera.translation.x - event.delta.x * settings_sensitivity;
            let new_y = camera.translation.y + event.delta.y * settings_sensitivity;

            if new_y > -5.0 && new_y < 5.0 {
                camera.translation.y = new_y;
            }
            if new_x > -10.0 && new_x < 10.0 {
                camera.translation.x = new_x;
            }
        }
    }
    // Orbit
    else if mouse_input.pressed(MouseButton::Right) {
        for event in mouse_evr.iter() {
            rotation_move += event.delta;
        }

        if rotation_move.length_squared() > 0.0 {}
    }

    // if input.pressed(MouseButton::Left) {
    //     println!("Left mouse button down");
    //     let new_y = camera.translation.y + 1.0;

    //     if new_y > 1.0 && new_y < 15.0 {
    //         camera.translation.y = new_y;
    //     }
    // }

    // if input.just_pressed(MouseButton::Left) {
    //     println!("Left mouse button pressed");
    //     query.get_single_mut().unwrap().translation.x += 1.0;
    // }
}
