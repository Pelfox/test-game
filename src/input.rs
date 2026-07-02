//! This module represents and implements input events, state and handling.

use winit::{
    event::{DeviceEvent, ElementState, WindowEvent},
    keyboard::PhysicalKey,
};

/// Describes a single unique input event.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputEvent {
    /// Indicates that focus of the outer window has changed. Boolean indicates
    /// whether window is now active (focused) or not.
    FocusChange(bool),
    /// Indicates that the key or a set of keys of the keyboard are pressed or
    /// released.
    KeyboardKeyPress {
        /// Physical keyboard key that was pressed.
        ///
        /// This value already includes all modifiers and is
        /// language-independent.
        key: PhysicalKey,
        /// State of the key.
        state: ElementState,
    },
    /// Represents mouse movement inside the window.
    ///
    /// Coordinates are delta to the previous location.
    MouseMovement {
        /// Position delta alongside X axis.
        x: f64,
        /// Position delta alongside Y axis.
        y: f64,
    },
    /// Represents cursor movement inside the window.
    ///
    /// Coordinates are absolute coordinates for cursor's position in the
    /// window.
    CursorMovement {
        /// Absolute cursor coordinate on the X axis.
        x: f64,
        /// Absolute cursor coordinate on the Y axis.
        y: f64,
    },
}

/// Describes the event handler function for input events. This function will
/// receive all events that are produced by the input state changes, and it's
/// up to the callback implementation to extract target event.
///
/// Event handler will receive the event itself, alongside the game state and
/// the input state itself.
pub type InputEventHandler<GS, IS> =
    for<'e, 'gs, 'is> fn(&'e InputEvent, &'gs mut GS, &'is mut IS) -> anyhow::Result<()>;

/// Holds the current state of all user inputs (keyboard, mouse).
#[derive(Default)]
pub struct InputState<GS> {
    handlers: Vec<InputEventHandler<GS, Self>>,
}

impl<GS> InputState<GS> {
    /// Registers a new input event handler.
    pub fn register_handler(&mut self, handler: InputEventHandler<GS, Self>) {
        self.handlers.push(handler);
    }

    /// Cleans up the current state, indicating the end of the frame.
    ///
    /// This method should only be called by the state controller once per
    /// frame.
    pub fn end_frame(&mut self) {}

    fn emit_event(&mut self, game_state: &mut GS, event: InputEvent) {
        for i in 0..self.handlers.len() {
            let handler = self.handlers[i];
            if let Err(e) = handler(&event, game_state, self) {
                log::error!("Failed to handle input event: {e:?}");
            }
        }
    }

    /// Processes the given window event, passing it down to handlers.
    pub fn on_window_event(&mut self, game_state: &mut GS, event: WindowEvent) {
        match event {
            WindowEvent::Focused(focused) => {
                self.emit_event(game_state, InputEvent::FocusChange(focused));
            }
            WindowEvent::KeyboardInput { event, .. } => {
                // TODO: Ideally, I think, we should provide modifiers for the
                // key too? It is possible to inherit modifiers from some
                // physical keys.
                let event = InputEvent::KeyboardKeyPress {
                    key: event.physical_key,
                    state: event.state,
                };
                self.emit_event(game_state, event);
            }
            WindowEvent::CursorMoved { position, .. } => {
                let event = InputEvent::CursorMovement {
                    x: position.x,
                    y: position.y,
                };
                self.emit_event(game_state, event);
            }
            // WindowEvent::MouseInput {
            //     device_id,
            //     state,
            //     button,
            // } => todo!(),
            // WindowEvent::MouseWheel {
            //     device_id,
            //     delta,
            //     phase,
            // } => todo!(),
            _ => {}
        }
    }

    /// Processes given device event, triggering input handlers.
    pub fn on_device_event(&mut self, game_state: &mut GS, is_focused: bool, event: &DeviceEvent) {
        if !is_focused {
            return;
        }

        match event {
            DeviceEvent::MouseMotion { delta } => {
                let event = InputEvent::MouseMovement {
                    x: delta.0,
                    y: delta.1,
                };
                self.emit_event(game_state, event);
            }
            _ => {}
        }
    }
}
