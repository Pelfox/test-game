use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

use crate::renderer::GameRenderer;

#[derive(Default)]
pub struct GameState {
    renderer: Option<crate::renderer::GameRenderer>,
}

impl ApplicationHandler for GameState {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.renderer.is_some() {
            return;
        }

        let attributes = Window::default_attributes().with_title("My Game");
        match event_loop.create_window(attributes) {
            Ok(window) => {
                match pollster::block_on(GameRenderer::new(window)) {
                    Ok(renderer) => self.renderer = Some(renderer),
                    Err(e) => log::error!("Failed to initialize renderer: {e:?}"),
                };
            }
            Err(e) => log::error!("Failed to initialize the window: {e:?}"),
        };
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                if let Some(renderer) = self.renderer.as_ref() {
                    renderer.render();
                }
            }
            _ => {}
        }
    }
}
