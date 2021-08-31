mod audio;
mod camera;
mod channel;
mod config;
mod input;
mod instance;
mod print;
mod render;
mod render_op;
mod render_pipleline;
mod resize;
mod setup;
mod state;
mod uniforms;
mod vertex;
use crate::config::{CameraConfig, Config};
use crate::state::State;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use futures::executor::block_on;

fn main() {
    env_logger::init();
    let config = Config {
        filename: "kintaro".into(),
        volume: 0.5,
        window_size: (1600, 1000),
        // camera: CameraConfig {
        // position: (0.0, 0.0, 65.0),
        // yaw: -90.0,
        // pitch: 0.0,
        // },
        camera: CameraConfig {
            position: (-48.2, -3.5, 16.0),
            yaw: -50.0,
            pitch: 8.0,
        },
    };
    let title = env!("CARGO_PKG_NAME");
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(winit::dpi::PhysicalSize {
            width: config.window_size.0,
            height: config.window_size.1,
        })
        .with_title(title)
        // .with_fullscreen(Some(Fullscreen::Borderless(None)))
        .build(&event_loop)
        .expect("Unable to create window");

    let op_stream = crate::render_op::OpStream::from_json(&config.filename);
    let mut state = block_on(State::new(&window, op_stream, &config));
    let (_stream, _stream_handle) = crate::audio::play_audio(&config);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::MainEventsCleared => window.request_redraw(),
            Event::DeviceEvent { ref event, .. } => {
                state.input(event);
            }
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                // dbg!(event);
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { input, .. } => match input {
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        } => {
                            *control_flow = ControlFlow::Exit;
                        }
                        _ => state.keyboard_input(event),
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

            Event::RedrawRequested(_) => {
                match state.render() {
                    Ok(_) => {}
                    // Recreate the swap_chain if lost
                    Err(wgpu::SwapChainError::Lost) => state.resize(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            _ => {}
        }
    });
}
