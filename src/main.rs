mod state;
mod vertex;
use crate::state::State;
use crate::vertex::Vertex;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use futures::executor::block_on;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .expect("Unable to create window");
    const VERTICES: &[Vertex] = &[
        Vertex {
            position: [0.0, 0.5, 0.0],
            color: [1.0, 0.0, 0.0],
        },
        Vertex {
            position: [-0.5, -0.5, 0.0],
            color: [0.0, 1.0, 0.0],
        },
        Vertex {
            position: [0.5, -0.5, 0.0],
            color: [0.0, 0.0, 1.0],
        },
    ];
    let mut state = block_on(State::new(&window, VERTICES));

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(_) => {
            state.update();
            match state.render() {
                Ok(_) => {}
                Err(wgpu::SwapChainError::Lost) => state.resize(state.size),
                Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => println!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            if !state.input(event) {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { input, .. } => match input {
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        _ => {}
                    },
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    });
}
