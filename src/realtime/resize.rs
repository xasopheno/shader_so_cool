use crate::{
    canvas::Canvas, instance::make_instances_and_instance_buffer, realtime::RealTimeState,
};

impl RealTimeState {
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.canvas = Canvas::init((new_size.width, new_size.height));

        let (instances, instance_buffer) =
            make_instances_and_instance_buffer(0, (new_size.width, new_size.height), &self.device);
        self.renderpass.instances = instances;
        self.renderpass.instance_buffer = instance_buffer;

        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;

        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
        self.camera
            .projection
            .resize(new_size.width, new_size.height);
    }
}
