use crate::{clock::ClockResult, shared::update};

use super::PrintState;

impl PrintState {
    pub fn update(&mut self, time: ClockResult) {
        update(
            time,
            &mut self.renderpass,
            &self.device,
            &self.queue,
            (self.size.0, self.size.1),
            &self.canvas,
        )
    }
}
