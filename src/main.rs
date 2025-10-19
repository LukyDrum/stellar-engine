use std::sync::Arc;

use stellar_engine::{
    math::Vector2,
    rendering::{
        RendererState,
        shapes::{Shape, Triangle},
    },
};
use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

#[derive(Default)]
pub struct App {
    state: Option<RendererState>,
}

impl ApplicationHandler<RendererState> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        self.state = Some(pollster::block_on(RendererState::new(window)).unwrap());
    }

    #[allow(unused_mut)]
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: RendererState) {
        self.state = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                let _ = state.render();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state,
                        ..
                    },
                ..
            } => match (code, state.is_pressed()) {
                (KeyCode::Escape, true) => event_loop.exit(),
                // Add shape on space
                (KeyCode::Space, true) => {
                    if let Some(state) = &mut self.state {
                        let rand_vector2 = || {
                            let (width, height) = state.window_size();
                            let x = fastrand::u32(0..width) as f32;
                            let y = fastrand::u32(0..height) as f32;
                            Vector2::new(x, y)
                        };

                        let shape = Shape::Triangle(Triangle {
                            points: [rand_vector2(), rand_vector2(), rand_vector2()],
                        });

                        state.render_queue.add(shape);
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}

pub fn run() -> anyhow::Result<()> {
    env_logger::init();

    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::default();
    event_loop.run_app(&mut app)?;

    Ok(())
}

fn main() {
    run().unwrap()
}
