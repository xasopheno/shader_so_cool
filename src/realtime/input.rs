use crate::realtime::RealTimeState;
use winit::event::*;

impl RealTimeState {
    pub fn keyboard_input(&mut self, event: &WindowEvent) {
        if let WindowEvent::KeyboardInput {
            input:
                winit::event::KeyboardInput {
                    state,
                    virtual_keycode: Some(key),
                    ..
                },
            ..
        } = event
        {
            self.composition
                .handle_keyboard_input(*key, *state, &mut self.cameras)
        };
    }

    pub fn input(&mut self, event: &DeviceEvent) -> bool {
        match event {
            DeviceEvent::Key(KeyboardInput {
                virtual_keycode: Some(key),
                state,
                ..
            }) => self
                .cameras
                .current
                .controller
                .process_keyboard(*key, *state),
            DeviceEvent::MouseWheel { delta, .. } => {
                self.cameras.current.controller.process_scroll(&*delta);
                true
            }
            DeviceEvent::Button { button: _, state } => {
                self.mouse_pressed = *state == ElementState::Pressed;
                true
            }
            DeviceEvent::MouseMotion { delta } => {
                if self.mouse_pressed {
                    self.cameras
                        .current
                        .controller
                        .process_mouse(delta.0, delta.1);
                }
                true
            }
            _ => false,
        }
    }
}
