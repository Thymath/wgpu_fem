mod state_wgpu;
mod window;

use window::App;

fn main() {
    let instance_descriptor = wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    };
    let instance = wgpu::Instance::new(instance_descriptor);

    for adapter in instance.enumerate_adapters(wgpu::Backends::all()) {
        println!("{:?}", adapter.get_info());
    }

    let event_loop = winit::event_loop::EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    // event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);

    let mut app = App::default();

    let _ = event_loop.run_app(&mut app);
}
