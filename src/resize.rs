use crate::{
    instance::make_instances_and_instance_buffer,
    state::{canvas_info, State},
};

impl State {
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.canvas = canvas_info(new_size);

        let (instances, instance_buffer) =
            make_instances_and_instance_buffer(0, new_size, &self.device);
        self.instances = instances;
        self.instance_buffer = instance_buffer;

        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;

        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
        self.projection.resize(new_size.width, new_size.height);
    }
}
