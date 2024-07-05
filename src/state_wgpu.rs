use crate::pipeline_builder::PipelineBuilder;
use winit::dpi::PhysicalSize;

#[derive(Debug)]
pub struct State<'a> {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: PhysicalSize<u32>,
    pub window: winit::window::Window,
    render_pipeline: wgpu::RenderPipeline,
}

impl<'a> State<'a> {
    pub async fn new(event_loop: &winit::event_loop::ActiveEventLoop) -> Self {
        let window = event_loop
            .create_window(winit::window::Window::default_attributes())
            .unwrap();

        let size = window.inner_size();

        let instance_descriptor = wgpu::InstanceDescriptor {
            backends: wgpu::Backends::VULKAN,
            ..Default::default()
        };
        let instance = wgpu::Instance::new(instance_descriptor);

        let target = unsafe { wgpu::SurfaceTargetUnsafe::from_window(&window) }.unwrap();
        let surface = unsafe { instance.create_surface_unsafe(target) }.unwrap();

        let adapter_descriptor = wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        };
        let adapter = instance
            .request_adapter(&adapter_descriptor)
            .await
            .expect("Failed to finde an appropiate adapter");

        let device_descriptor = wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            label: Some("Device"),
        };
        let (device, queue) = adapter
            .request_device(&device_descriptor, None)
            .await
            .expect("Failed to create device");

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        let mut pipeline_builder = PipelineBuilder::new();
        pipeline_builder.set_shader_module("shaders/shader.wgsl", "vs_main", "fs_main");
        pipeline_builder.set_pixel_format(config.format);
        let render_pipeline = pipeline_builder.build_pipeline(&device);

        Self {
            instance,
            surface,
            device,
            queue,
            config,
            size,
            window,
            render_pipeline,
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let drawable = self.surface.get_current_texture()?;
        let image_view_descriptor = wgpu::TextureViewDescriptor::default();
        let image_view = drawable.texture.create_view(&image_view_descriptor);

        let command_encoder_descriptor = wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        };
        let mut command_encoder = self
            .device
            .create_command_encoder(&command_encoder_descriptor);

        let color_attachment = wgpu::RenderPassColorAttachment {
            view: &image_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.25,
                    g: 0.0,
                    b: 0.5,
                    a: 0.0,
                }),
                store: wgpu::StoreOp::Store,
            },
        };
        let render_pass_descriptor = wgpu::RenderPassDescriptor {
            label: Some("Renderpass"),
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        };

        {
            let mut renderpass = command_encoder.begin_render_pass(&render_pass_descriptor);

            renderpass.set_pipeline(&self.render_pipeline);
            renderpass.draw(0..3, 0..1);
        }

        self.queue.submit(std::iter::once(command_encoder.finish()));

        drawable.present();
        Ok(())
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn update_surface(&mut self) {
        let target = unsafe { wgpu::SurfaceTargetUnsafe::from_window(&self.window) }.unwrap();

        self.surface = unsafe { self.instance.create_surface_unsafe(target) }.unwrap();
    }
}
