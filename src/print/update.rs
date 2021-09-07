use crate::shared::update;

use super::PrintState;

impl PrintState {
    pub fn update(&mut self) {
        update(
            &mut self.clock,
            &mut self.renderpass,
            &self.device,
            &self.queue,
            (self.size.0, self.size.1),
            &mut self.camera,
            &mut self.camera_controller,
            &self.projection,
            &self.canvas,
            &mut self.op_stream,
        )
    }
}
