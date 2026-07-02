//! Representation of the game application itself as well as state handling.

use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, DeviceId, WindowEvent},
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

use crate::{
    input::{InputEventHandler, InputState},
    renderer::GameRenderer,
};

/// Holds the current state of the game.
#[derive(Default)]
pub struct GameState {
    /// Instance of the game renderer.
    ///
    /// This field is initialized when window is first created, and can be
    /// optionally mutated afterwards, but it does not re-initialize more than
    /// once. This might be crucial for some applications.
    pub renderer: Option<crate::renderer::GameRenderer>,
    /// Indicates whether game's window is currently focused or not.
    pub is_focused: bool,
}

/// Holds the central controller for the game's application.
#[derive(Default)]
pub struct GameApp {
    state: GameState,
    input_state: InputState<GameState>,
}

impl GameApp {
    /// Registers a new input handler for the underlying input state.
    pub fn register_input_handler(
        &mut self,
        handler: InputEventHandler<GameState, InputState<GameState>>,
    ) {
        self.input_state.register_handler(handler);
    }
}

impl ApplicationHandler for GameApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.renderer.is_some() {
            return;
        }

        let attributes = Window::default_attributes().with_title("My Game");
        match event_loop.create_window(attributes) {
            Ok(window) => {
                match pollster::block_on(GameRenderer::new(window)) {
                    Ok(renderer) => self.state.renderer = Some(renderer),
                    Err(e) => log::error!("Failed to initialize renderer: {e:?}"),
                };
            }
            Err(e) => log::error!("Failed to initialize the window: {e:?}"),
        };
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Focused(focused) => {
                self.state.is_focused = focused;
                self.input_state.on_window_event(&mut self.state, event);
                log::info!("Window focus changed to {focused}");
            }
            WindowEvent::RedrawRequested => {
                if let Some(renderer) = self.state.renderer.as_ref() {
                    renderer.render();
                }
            }
            _ => {
                self.input_state.on_window_event(&mut self.state, event);
            }
        }
    }

    fn device_event(&mut self, _: &ActiveEventLoop, _: DeviceId, event: DeviceEvent) {
        let is_focused = self.state.is_focused;
        self.input_state
            .on_device_event(&mut self.state, is_focused, &event);
    }
}
