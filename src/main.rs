fn main() {
    let instance_descriptor = wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    };
    let instance = wgpu::Instance::new(instance_descriptor);

    for adapter in instance.enumerate_adapters(wgpu::Backends::all()) {
        println!("{:?}", adapter.get_info());
    }
}
