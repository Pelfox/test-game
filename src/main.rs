use winit::{
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
};

use crate::{
    app::{GameApp, GameAppDescriptor},
    input::InputEvent,
    renderer::CameraPositionMovement,
};

pub mod app;
pub mod input;
pub mod math;
pub mod renderer;
pub mod scene;

/// Sensitivity of the mouse.
const MOUSE_SENSITIVITY: f32 = 0.002f32;

fn main() {
    pretty_env_logger::init();

    let event_loop = match EventLoop::new() {
        Ok(event_loop) => event_loop,
        Err(e) => {
            log::error!("Failed to create event loop: {e:?}");
            return;
        }
    };
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut game_app = GameApp::new(GameAppDescriptor {
        camera_movement_delta: 100.0,
        ..Default::default()
    });

    game_app.register_input_handler(|event, game_state, _| {
        match event {
            InputEvent::MouseMovement { x, y } => {
                let delta_x = *x as f32 * MOUSE_SENSITIVITY;
                let delta_y = *y as f32 * MOUSE_SENSITIVITY;
                if let Some(renderer) = game_state.renderer.as_mut() {
                    renderer.update_camera_direction(delta_x, delta_y);
                }
            }
            InputEvent::KeyboardKeyPress { keys, .. } => {
                let pressed = |code: KeyCode| keys.contains(&PhysicalKey::Code(code));
                game_state.camera_movement = CameraPositionMovement {
                    forward: pressed(KeyCode::KeyW),
                    backward: pressed(KeyCode::KeyS),
                    left: pressed(KeyCode::KeyA),
                    right: pressed(KeyCode::KeyD),
                    up: pressed(KeyCode::Space),
                    down: pressed(KeyCode::ShiftLeft),
                };
            }
            _ => {}
        }

        Ok(())
    });

    if let Err(e) = event_loop.run_app(&mut game_app) {
        log::error!("Failed to start application: {e:?}");
    }
}
