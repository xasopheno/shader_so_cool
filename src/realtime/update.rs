use crate::{
    clock::ClockResult,
    realtime::RealTimeState,
    shared::{update, RenderPassInput},
};

impl RealTimeState {
    pub fn _update(&mut self, time: ClockResult, renderpass: &mut RenderPassInput) {
        update(
            time,
            renderpass,
            &self.device,
            &self.queue,
            (self.size.width, self.size.height),
            &self.canvas,
        )
    }
}
