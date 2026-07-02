use winit::event_loop::{ControlFlow, EventLoop};

use crate::{app::GameApp, input::InputEvent};

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

    let mut game_app = GameApp::default();
    game_app.register_input_handler(|event, game_state, _| {
        match event {
            InputEvent::MouseMovement { x, y } => {
                let delta_x = *x as f32 * MOUSE_SENSITIVITY;
                let delta_y = *y as f32 * MOUSE_SENSITIVITY;
                if let Some(renderer) = game_state.renderer.as_mut() {
                    renderer.update_camera_direction(delta_x, delta_y);
                    renderer.render();
                    log::info!("Updating camera: delta_x={delta_x:?}, delta_y={delta_y:?}");
                }
            }
            _ => {}
        }

        Ok(())
    });

    if let Err(e) = event_loop.run_app(&mut game_app) {
        log::error!("Failed to start application: {e:?}");
    }
}
