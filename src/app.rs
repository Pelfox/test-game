//! Representation of the game application itself as well as state handling.

use std::time::Instant;

use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, DeviceId, WindowEvent},
    event_loop::ActiveEventLoop,
    window::{CursorGrabMode, Window, WindowId},
};

use crate::{
    input::{InputEventHandler, InputState},
    renderer::{CameraPositionMovement, GameRenderer},
};

/// Holds the current state of the game.
pub struct GameState {
    /// Instance of the game renderer.
    ///
    /// This field is initialized when window is first created, and can be
    /// optionally mutated afterwards, but it does not re-initialize more than
    /// once. This might be crucial for some applications.
    pub renderer: Option<crate::renderer::GameRenderer>,

    /// Indicates whether game's window is currently focused or not.
    pub is_focused: bool,

    /// Time elapsed from the last frame, in seconds.
    ///
    /// This variable should be used for all time-based updates independent of
    /// the frame rate (for example, movement, animations, etc).
    pub delta_time: f32,

    /// Represents camera's position movement for the next frame.
    ///
    /// Event handler should update this on keyboard event. Game state will
    /// automatically apply the movement on the next frame.
    pub camera_movement: CameraPositionMovement,

    last_frame_time: Instant,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            renderer: None,
            is_focused: false,
            delta_time: 0.0,
            camera_movement: CameraPositionMovement::default(),
            last_frame_time: Instant::now(),
        }
    }
}

/// Describes the initialization options for the [GameApp].
pub struct GameAppDescriptor {
    /// Delta that should be applied when camera is moved.
    ///
    /// You can set up camera's movement through [CameraPositionMovement].
    pub camera_movement_delta: f32,

    /// Whether we should try and lock up the cursor inside the window.
    pub try_lock_cursor: bool,

    /// Whether the cursor should be shown.
    pub show_cursor: bool,
}

impl Default for GameAppDescriptor {
    fn default() -> Self {
        Self {
            camera_movement_delta: 0.0,
            try_lock_cursor: true,
            show_cursor: false,
        }
    }
}

/// Holds the central controller for the game's application.
#[derive(Default)]
pub struct GameApp {
    state: GameState,
    descriptor: GameAppDescriptor,
    input_state: InputState<GameState>,
}

impl GameApp {
    /// Creates a new game app instance.
    pub fn new(descriptor: GameAppDescriptor) -> Self {
        let mut app = Self::default();
        app.descriptor = descriptor;
        app
    }

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
                if self.descriptor.try_lock_cursor {
                    if let Err(e) = window.set_cursor_grab(CursorGrabMode::Confined) {
                        log::error!("Could not confine the cursor: {e:?}");
                        if let Err(e) = window.set_cursor_grab(CursorGrabMode::Locked) {
                            log::error!("Could not lock the cursor: {e:?}");
                        }
                    }
                }
                window.set_cursor_visible(self.descriptor.show_cursor);

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
                let now = Instant::now();
                self.state.delta_time = (now - self.state.last_frame_time).as_secs_f32();
                self.state.last_frame_time = now;

                if let Some(renderer) = self.state.renderer.as_mut() {
                    renderer.update_camera_position(
                        self.state.camera_movement,
                        self.descriptor.camera_movement_delta,
                        self.state.delta_time,
                    );
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

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        // We should not call renderer if we are about to exit.
        if event_loop.exiting() {
            return;
        }

        if let Some(ref renderer) = self.state.renderer {
            renderer.request_redraw();
        }
    }
}
