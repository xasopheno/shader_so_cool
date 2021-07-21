mod state;
mod vertex;
use crate::state::State;
use crate::vertex::Vertex;
use rand::prelude::*;
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
    // const VERTICES: &[Vertex] = &[
    // Vertex {
    // position: [-0.0868241, 0.49240386, 0.0],
    // color: [0.2, 0.0, 0.5],
    // },
    // Vertex {
    // position: [-0.49513406, 0.06958647, 0.0],
    // color: [0.5, 1.0, 0.5],
    // },
    // Vertex {
    // position: [-0.21918549, -0.44939706, 0.0],
    // color: [0.1, 0.0, 0.8],
    // },
    // Vertex {
    // position: [0.35966998, -0.3473291, 0.0],
    // color: [0.8, 0.0, 0.2],
    // },
    // Vertex {
    // position: [0.44147372, 0.2347359, 0.0],
    // color: [0.5, 0.4, 0.2],
    // },
    // Vertex {
    // position: [0.7, 0.8, 0.0],
    // color: [0.1, 0.2, 0.1],
    // },
    // ];
    let mut rng = rand::thread_rng();

    let mut r = || rng.gen::<f32>() * 2.0 - 1.0;

    let vertices: Vec<Vertex> = (0..8).map(|_| Vertex::new_random()).collect();

    let mut num = || rng.gen_range(0..vertices.len() as u16);

    // let INDICES: &[u16] = &[0, 1, 4, 3, 6, 2, 4, 2, 1, 4, 5, 0];
    let indices: Vec<u16> = (0..100).map(|_| num()).collect();

    let mut state = block_on(State::new(
        &window,
        &mut vertices.as_slice(),
        &indices.as_slice(),
    ));

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
