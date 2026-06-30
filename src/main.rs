use winit::event_loop::{ControlFlow, EventLoop};

pub mod math;
pub mod renderer;
pub mod scene;
pub mod state;

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

    let mut game_app = state::GameState::default();
    if let Err(e) = event_loop.run_app(&mut game_app) {
        log::error!("Failed to start application: {e:?}");
    }
}
