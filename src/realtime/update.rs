use crate::{realtime::RealTimeState, shared::update};

impl RealTimeState {
    pub fn update(&mut self) {
        update(
            &mut self.clock,
            &mut self.renderpass,
            &self.device,
            &self.queue,
            (self.size.width, self.size.height),
            &mut self.camera,
            &self.canvas,
            &mut self.op_stream,
        )
    }
}
