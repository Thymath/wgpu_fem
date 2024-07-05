use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

use crate::state_wgpu::State;

#[derive(Default)]
pub struct App<'a> {
    state: Option<State<'a>>,
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.state = Some(pollster::block_on(State::new(event_loop)));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed. Stopping.");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.state.as_ref().unwrap().window.request_redraw();
            }
            _ => (),
        }
    }
}
