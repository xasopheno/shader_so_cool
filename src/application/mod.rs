use crate::config::Config;
use crate::print::PrintState;
use crate::realtime::gui::GuiRepaintSignal;
use crate::realtime::RealTimeState;
use crate::renderable::RenderableConfig;
use colored::*;
use cradle::prelude::*;
use rodio::{OutputStream, OutputStreamHandle};
use std::collections::HashMap;
use std::io::Write;
use std::str::FromStr;
use weresocool::error::Error;
use weresocool::generation::parsed_to_render::AudioVisual;
use weresocool::generation::{RenderReturn, RenderType};
use weresocool::interpretable::{InputType, Interpretable};
use winit::dpi::PhysicalSize;
#[allow(unused_imports)]
use winit::window::Fullscreen;
use winit::{event::*, event_loop::ControlFlow, window::WindowBuilder};

pub type AvMap = HashMap<String, AudioVisual>;
pub type AudioStreams = Vec<rodio::OutputStream>;
pub type AudioStreamHandles = Vec<rodio::Sink>;

pub fn run<'a>(filename: &str, config: Config<'static>) -> Result<(), Error> {
    println!("preparing for audiovisualization: {}", &filename);
    let mut av_map: AvMap = HashMap::new();
    let mut audio_streams: AudioStreams = vec![];
    let mut audio_stream_handles: AudioStreamHandles = vec![];

    for c in config.renderable_configs.iter() {
        match c {
            RenderableConfig::EventStreams(e) => {
                let result = get_audiovisual_data(&e.socool_path);

                match result {
                    Ok(r) => {
                        let (stream, stream_handle) = crate::audio::setup_audio(&config, &r.audio);
                        audio_streams.push(stream);
                        audio_stream_handles.push(stream_handle);

                        av_map.insert(e.socool_path.to_string(), r);
                    }
                    Err(e) => return Err(e),
                }
            }
            _ => {}
        }
    }

    // let av = av_map.get(filename).unwrap();
    if std::env::args().find(|x| x == "--print").is_some() {
        // println!("{}", "\n\n\n:::::<<<<<*****PRINTING*****>>>>>:::::".blue());

        // let n_frames = (av.length * 40.0).floor() as usize + 100;

        // println!("{}", format!("Number Frames: {}", n_frames).green());

        // print(config, &av, n_frames)?;
        // write_audio_to_file(
        // &av.audio.as_slice(),
        // std::path::PathBuf::from_str("kintaro.wav")
        // .expect("unable to create pathbuf for kintaro.wav"),
        // );

        // let command_join_audio_and_video = "ffmpeg -framerate 40 -pattern_type glob -i out/*.png -i kintaro.wav -c:a copy -shortest -c:v libx264 -r 40 -pix_fmt yuv420p out.mov";

        // run!(Stdin("yes"), %command_join_audio_and_video);
    } else {
        println!("****REALTIME****");
        realtime(config, av_map, audio_stream_handles)?;
    }
    Ok(())
}

pub fn write_audio_to_file(audio: &[u8], filename: std::path::PathBuf) {
    let mut file = std::fs::File::create(filename.clone()).unwrap();
    file.write_all(audio).unwrap();
    println!("Audio file written: {}", filename.display().to_string());
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

fn print(mut config: Config<'static>, av: &AvMap, n_frames: usize) -> Result<(), Error> {
    let mut state = async_std::task::block_on(PrintState::init(&mut config, av))?;
    for i in 0..n_frames {
        async_std::task::block_on(state.render())
            .expect(format!("Unable to render frame: {}", i).as_str());
    }
    Ok(())
}

fn realtime<'a>(
    mut config: Config<'static>,
    av_map: AvMap,
    stream_handles: AudioStreamHandles,
) -> Result<(), Error> {
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

    let repaint_signal = std::sync::Arc::new(GuiRepaintSignal(std::sync::Mutex::new(
        event_loop.create_proxy(),
    )));

    let mut state = RealTimeState::init(
        &window,
        &mut config,
        repaint_signal,
        av_map,
        stream_handles,
        // &av
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
