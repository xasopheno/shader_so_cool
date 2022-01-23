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
use weresocool::generation::{Op4D, RenderReturn, RenderType};
use weresocool::interpretable::{InputType, Interpretable};
use winit::dpi::PhysicalSize;
#[allow(unused_imports)]
use winit::window::Fullscreen;
use winit::{event::*, event_loop::ControlFlow, window::WindowBuilder};

pub type AvMap = HashMap<String, Visual>;

#[derive(Clone, Debug)]
/// AudioVisual is the datatype for audiovisualization
pub struct Visual {
    /// Composition name
    pub name: String,
    /// length of seconds of composition
    pub length: f32,
    /// audio data
    pub visual: Vec<Op4D>,
}

pub type Audio = Vec<u8>;
// pub struct Audio {
// /// Composition name
// pub name: String,
// /// length of seconds of composition
// pub length: f32,
// /// audio data
// pub audio: Vec<u8>,
// }

/// Sum a Vec<Vec<u8> to a single Vec<u8>.
///
fn split_audio_visual(av: AudioVisual) -> (Audio, Visual) {
    (
        av.audio,
        Visual {
            name: av.name.clone(),
            length: av.length,
            visual: av.visual,
        },
    )
}

pub fn sum_all_waveforms(mut vec_wav: Vec<Vec<u8>>) -> Vec<u8> {
    // Sort the vectors by length
    sort_vecs(&mut vec_wav);

    // Get the length of the longest vector
    let max_len = vec_wav[0].len();

    let mut result = vec![0; max_len];

    for wav in vec_wav {
        sum_vec(&mut result, &wav[..]);
    }

    result
}

/// Sort a Vec of Vec<u8> by length.
fn sort_vecs(vec_wav: &mut Vec<Vec<u8>>) {
    vec_wav.sort_unstable_by(|a, b| b.len().cmp(&a.len()));
}

/// Sum two vectors. Assumes vector a is longer than or of the same length
/// as vector b.
pub fn sum_vec(a: &mut Vec<u8>, b: &[u8]) {
    for (ai, bi) in a.iter_mut().zip(b) {
        *ai += *bi;
    }
}

pub fn run<'a>(filename: &str, config: Config<'static>) -> Result<(), Error> {
    println!("preparing for audiovisualization: {}", &filename);
    let mut av_map: AvMap = HashMap::new();
    let mut audios: Vec<Audio> = vec![];

    for c in config.renderable_configs.iter() {
        match c {
            RenderableConfig::EventStreams(e) => {
                let result = get_audiovisual_data(&e.socool_path);

                match result {
                    Ok(r) => {
                        let (a, v) = split_audio_visual(r);
                        audios.push(a);
                        av_map.insert(e.socool_path.to_string(), v);
                    }
                    Err(e) => return Err(e),
                }
            }
            _ => {}
        }
    }

    let audio = sum_all_waveforms(audios);
    let (_stream, stream_handle) = crate::audio::setup_audio(&config, &audio);

    if std::env::args().find(|x| x == "--print").is_some() {
        println!("{}", "\n\n\n:::::<<<<<*****PRINTING*****>>>>>:::::".blue());

        let max_frames = match av_map.values().max_by_key(|v| v.length as usize) {
            Some(mf) => mf.length as usize,
            None => 1000,
        };

        let n_frames = (max_frames * 40) + 100;

        println!("{}", format!("Number Frames: {}", n_frames).green());

        print(config, &av_map, n_frames)?;

        write_audio_to_file(
            &audio.as_slice(),
            std::path::PathBuf::from_str("kintaro.wav")
                .expect("unable to create pathbuf for kintaro.wav"),
        );

        let command_join_audio_and_video = "ffmpeg -framerate 40 -pattern_type glob -i out/*.png -i kintaro.wav -c:a copy -shortest -c:v libx264 -r 40 -pix_fmt yuv420p out.mov";

        run!(Stdin("yes"), %command_join_audio_and_video);
    } else {
        println!("****REALTIME****");
        realtime(config, av_map, stream_handle)?;
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
    stream_handles: rodio::Sink,
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
