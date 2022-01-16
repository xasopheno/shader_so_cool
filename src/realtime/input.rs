use crate::realtime::RealTimeState;
use winit::event::*;

impl<'a> RealTimeState<'a> {
    pub fn keyboard_input(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { input, .. } => match input {
                KeyboardInput {
                    state,
                    virtual_keycode: Some(key),
                    ..
                } => {
                    self.composition
                        .camera
                        .controller
                        .process_keyboard(*key, *state);
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn input(&mut self, event: &DeviceEvent) -> bool {
        match event {
            DeviceEvent::Key(KeyboardInput {
                virtual_keycode: Some(key),
                state,
                ..
            }) => self
                .composition
                .camera
                .controller
                .process_keyboard(*key, *state),
            DeviceEvent::MouseWheel { delta, .. } => {
                self.composition.camera.controller.process_scroll(&*delta);
                true
            }
            DeviceEvent::Button { button: _, state } => {
                self.mouse_pressed = *state == ElementState::Pressed;
                true
            }
            DeviceEvent::MouseMotion { delta } => {
                if self.mouse_pressed {
                    self.composition
                        .camera
                        .controller
                        .process_mouse(delta.0, delta.1);
                }
                true
            }
            _ => false,
        }
    }
}
