use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;

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
        let state = self.state.as_mut().unwrap();
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed. Stopping.");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Outdated | wgpu::SurfaceError::Lost) => {
                        state.update_surface();
                        state.resize(state.size);
                    }
                    Err(e) => eprintln!("{:?}", e),
                }
                state.window.request_redraw();
            }
            WindowEvent::Resized(new_size) => {
                state.resize(new_size.into());
            }
            _ => (),
        }
    }
}
