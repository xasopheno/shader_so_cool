mod audio;
mod camera;
mod canvas;
mod clock;
mod config;
mod instance;
mod print;
mod realtime;
mod render_op;
mod shared;
mod toy;
mod uniforms;
mod vertex;
use crate::config::Config;
use crate::print::PrintState;
use crate::realtime::render::ExampleRepaintSignal;
use crate::realtime::RealTimeState;
#[allow(unused_imports)]
use winit::window::Fullscreen;
use winit::{event::*, event_loop::ControlFlow, window::WindowBuilder};

use futures::executor::block_on;

fn main() {
    let print_it = std::env::args()
        .into_iter()
        .any(|arg| if arg == "--print" { true } else { false });

    if print_it {
        println!("****PRINTING****");
        print();
    } else {
        println!("****REALTIME****");
        realtime();
    }
}

fn print() {
    let config = Config::new();
    let mut state = block_on(PrintState::init(config));
    for _ in 0..20000 {
        // for _ in 0..1000 {
        block_on(state.render());
    }
}

fn realtime() {
    env_logger::init();
    let config = Config::new();
    let title = env!("CARGO_PKG_NAME");
    let event_loop = winit::event_loop::EventLoop::with_user_event();
    let window = WindowBuilder::new()
        .with_inner_size(winit::dpi::PhysicalSize {
            width: config.window_size.0,
            height: config.window_size.1,
        })
        .with_transparent(true)
        .with_title(title)
        // .with_fullscreen(Some(Fullscreen::Borderless(None)))
        .with_decorations(true)
        .build(&event_loop)
        .expect("Unable to create window");

    let repaint_signal = std::sync::Arc::new(ExampleRepaintSignal(std::sync::Mutex::new(
        event_loop.create_proxy(),
    )));

    let (mut _stream, stream_handle) = crate::audio::play_audio(&config);
    let mut state = RealTimeState::init(&window, &config, repaint_signal.clone(), stream_handle);
    state.play();

    event_loop.run(move |event, _, control_flow| {
        #[allow(unused_assignments)]
        // if state.gui.state.lock().unwrap().reset {
        // state.pause();
        // let (new_stream, new_stream_handle) = crate::audio::play_audio(&config);
        // stream = new_stream;

        // state =
        // RealTimeState::init(&window, &config, repaint_signal.clone(), new_stream_handle);
        // state.play();
        // // state.gui.state.reset = false;
        // }
        state.gui.platform.handle_event(&event);
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
                match state.render(&window) {
                    Ok(_) => {}
                    // Recreate the swap_chain if lost
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                };

                *control_flow = ControlFlow::Poll;
            }
            _ => {}
        }
    });
}
