mod audio;
mod camera;
mod canvas;
mod clock;
mod color;
mod config;
mod gen;
mod instance;
mod print;
mod realtime;
mod render_op;
mod save;
mod shared;
mod texture;
mod toy;
mod uniforms;
mod vertex;
use crate::print::PrintState;
use crate::realtime::render::ExampleRepaintSignal;
use crate::realtime::RealTimeState;
use crate::{config::Config, save::ConfigState};
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
    let saved = load_saved();
    let mut config = Config::new(Some(saved));
    let mut state = block_on(PrintState::init(&mut config));
    for i in 0..10000 {
        block_on(state.render()).expect(format!("Unable to render frame: {}", i).as_str());
    }
}

fn load_saved() -> ConfigState {
    let path = "../kintaro/saved.json";
    let saved_data = std::fs::read_to_string(path).expect("Unable to read file");
    let saved: ConfigState =
        serde_json::from_str(&saved_data).expect("nable to deserialize saved data");
    saved
}

fn realtime() {
    env_logger::init();
    let saved = load_saved();
    let mut config = Config::new(Some(saved));
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
    let mut state =
        RealTimeState::init(&window, &mut config, repaint_signal.clone(), stream_handle);
    state.play();

    event_loop.run(move |event, _, control_flow| {
        #[allow(unused_assignments)]
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
