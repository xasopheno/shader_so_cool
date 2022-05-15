use crate::{
    canvas::Canvas,
    // instance::make_instances_and_instance_buffer,
    realtime::RealTimeState,
};

impl RealTimeState {
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = (new_size.width, new_size.height);
            // self.frame.resize(&self.device, new_size);
            self.canvas = Canvas::init((new_size.width, new_size.height));

            // let (instances, instance_buffer) =
            // make_instances_and_instance_buffer(0, (new_size.width, new_size.height), &self.device);
            // self.renderpass.instances = instances;
            // self.renderpass.instance_buffer = instance_buffer;

            self.composition
                .camera
                .projection
                .resize(new_size.width, new_size.height);

            let config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                width: new_size.width,
                height: new_size.height,
                present_mode: wgpu::PresentMode::Fifo,
            };

            self.surface.surface.configure(&self.device, &config);
        }
    }
}
