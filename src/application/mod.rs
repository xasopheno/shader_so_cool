use crate::config::Config;
use crate::print::PrintState;
use crate::realtime::render::ExampleRepaintSignal;
use crate::realtime::RealTimeState;
use weresocool::error::Error;
use weresocool::generation::parsed_to_render::AudioVisual;
use weresocool::generation::{RenderReturn, RenderType};
use weresocool::interpretable::{InputType, Interpretable};
use winit::dpi::PhysicalSize;
#[allow(unused_imports)]
use winit::window::Fullscreen;
use winit::{event::*, event_loop::ControlFlow, window::WindowBuilder};

use futures::executor::block_on;

pub fn run(filename: &str, config: Config) -> Result<(), Error> {
    println!("preparing for audiovisualization: {}", &filename);
    let av = get_audiovisual_data(filename)?;
    let print_it = std::env::args()
        .into_iter()
        .any(|arg| if arg == "--print" { true } else { false });

    if print_it {
        println!("****PRINTING****");
        let n_frames = (av.length * 40.0).floor() as usize + 100;
        print(config, av, n_frames)?;
    } else {
        println!("****REALTIME****");
        realtime(config, av)?;
    }
    Ok(())
}

fn get_audiovisual_data(filename: &str) -> Result<AudioVisual, Error> {
    if let RenderReturn::AudioVisual(av) =
        InputType::Filename(&filename).make(RenderType::AudioVisual, None)?
    {
        Ok(av)
    } else {
        Err(Error::with_msg(format!("Error rendering {}", filename)))
    }
}

fn print(mut config: Config, av: AudioVisual, n_frames: usize) -> Result<(), Error> {
    let mut state = block_on(PrintState::init(&mut config, av))?;
    for i in 0..n_frames {
        block_on(state.render()).expect(format!("Unable to render frame: {}", i).as_str());
    }
    Ok(())
}

fn realtime(mut config: Config, av: AudioVisual) -> Result<(), Error> {
    env_logger::init();
    let title = env!("CARGO_PKG_NAME");
    let event_loop = winit::event_loop::EventLoop::with_user_event();
    let window = WindowBuilder::new()
        .with_inner_size(winit::dpi::PhysicalSize {
            width: config.window_size.0,
            height: config.window_size.1,
        })
        .with_transparent(false)
        .with_title(title)
        // .with_fullscreen(Some(Fullscreen::Borderless(None)))
        .with_decorations(true)
        .build(&event_loop)
        .expect("Unable to create window");

    let repaint_signal = std::sync::Arc::new(ExampleRepaintSignal(std::sync::Mutex::new(
        event_loop.create_proxy(),
    )));

    let (mut _stream, stream_handle) = crate::audio::play_audio(&config, &av.audio);
    let mut state = RealTimeState::init(
        &window,
        &mut config,
        repaint_signal.clone(),
        stream_handle,
        av,
    )?;
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
                    Err(wgpu::SurfaceError::Lost) => state.resize(PhysicalSize {
                        width: state.size.0,
                        height: state.size.1,
                    }),
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
    #[allow(unreachable_code)]
    Ok(())
}
