use crate::config::Config;
use crate::error::KintaroError;
use crate::print::PrintState;
use crate::realtime::gui::GuiRepaintSignal;
use crate::realtime::RealTimeState;
use crate::renderable::RenderableConfig;
use colored::*;
use cradle::prelude::*;
use rodio::OutputStream;
use std::collections::HashMap;
use std::io::Write;
use std::str::FromStr;
use weresocool::{
    error::Error,
    generation::parsed_to_render::AudioVisual,
    generation::{Op4D, RenderReturn, RenderType},
    interpretable::{InputType, Interpretable},
};

#[allow(unused_imports)]
use winit::{
    dpi::PhysicalSize, event::*, event_loop::ControlFlow, window::Fullscreen, window::WindowBuilder,
};

pub type VisualsMap = HashMap<String, Visual>;

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

/// Sum a Vec<Vec<u8> to a single Vec<u8>.
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

pub fn run(filename: &str, config: Config<'static>) -> Result<(), KintaroError> {
    println!("preparing for audiovisualization: {}", &filename);
    let mut av_map: VisualsMap = HashMap::new();
    let mut audios: Vec<Audio> = vec![];

    for c in config.frame_passes.iter().flat_map(|c| &c.renderables) {
        if let RenderableConfig::EventStreams(e) = c {
            let result = get_audiovisual_data(&e.socool_path)?;

            let (a, v) = split_audio_visual(result);
            audios.push(a);
            av_map.insert(e.socool_path.to_string(), v);
        }
    }

    let mut audio: Option<Vec<u8>> = None;
    let _stream: OutputStream;
    let mut stream_handle: Option<rodio::Sink> = None;
    if audios.len() > 0 {
        let a = sum_all_waveforms(audios);
        let (_s, s_h) = crate::audio::setup_audio(&config, &a);
        audio = Some(a);
        _stream = _s;
        stream_handle = Some(s_h);
    }

    if std::env::args().any(|x| x == "--print") {
        println!(
            "{}",
            "\n\n\n:::::<<<<<*****PRINTING*****>>>>>:::::".magenta()
        );

        let max_frames = match av_map.values().max_by_key(|v| v.length as usize) {
            Some(mf) => mf.length as usize,
            None => 1000,
        };

        let n_frames = (max_frames * 40) + 100;

        println!("{}", format!("Number Frames: {}", n_frames).green());

        print(config, &av_map, n_frames)?;

        if let Some(a) = audio {
            write_audio_to_file(
                a.as_slice(),
                std::path::PathBuf::from_str("kintaro.wav")
                    .expect("unable to create pathbuf for kintaro.wav"),
            )?;

            let command_join_audio_and_video = "ffmpeg -framerate 40 -pattern_type glob -i out/*.png -i kintaro.wav -c:a copy -shortest -c:v libx264 -r 40 -pix_fmt yuv420p out.mov";

            run!(Stdin("yes"), %command_join_audio_and_video);
        }
    } else {
        println!("****REALTIME****");
        realtime(config, av_map, stream_handle)?;
    }
    Ok(())
}

pub fn write_audio_to_file(audio: &[u8], filename: std::path::PathBuf) -> Result<(), KintaroError> {
    let mut file = std::fs::File::create(filename.clone())?;
    file.write_all(audio)?;
    println!("Audio file written: {}", filename.display());
    Ok(())
}

fn get_audiovisual_data(filename: &str) -> Result<AudioVisual, Error> {
    if let RenderReturn::AudioVisual(av) =
        InputType::Filename(filename).make(RenderType::AudioVisual, None)?
    {
        Ok(av)
    } else {
        Err(Error::with_msg(format!("Error rendering {}", filename)))
    }
}

fn print(
    mut config: Config<'static>,
    av: &VisualsMap,
    n_frames: usize,
) -> Result<(), KintaroError> {
    let mut state = async_std::task::block_on(PrintState::init(&mut config, av))?;
    for i in 0..n_frames {
        async_std::task::block_on(state.render())
            .unwrap_or_else(|_| panic!("Unable to render frame: {}", i));
    }
    Ok(())
}

fn realtime(
    mut config: Config<'static>,
    av_map: VisualsMap,
    stream_handles: Option<rodio::Sink>,
) -> Result<(), KintaroError> {
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
        // .with_decorations(true)
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
                    Err(KintaroError::WgpuSurfaceError(wgpu::SurfaceError::Lost)) => {
                        state.resize(PhysicalSize {
                            width: state.size.0,
                            height: state.size.1,
                        })
                    }
                    // The system is out of memory, we should probably quit
                    Err(KintaroError::WgpuSurfaceError(wgpu::SurfaceError::OutOfMemory)) => {
                        *control_flow = ControlFlow::Exit
                    }
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
