use crate::{clock::ClockResult, realtime::RealTimeState, shared::update};

impl RealTimeState {
    pub fn update(&mut self, time: ClockResult) {
        update(
            time,
            &mut self.renderpass,
            &self.device,
            &self.queue,
            (self.size.width, self.size.height),
            &self.canvas,
        )
    }
}
