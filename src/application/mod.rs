pub mod print;
pub mod realtime;
pub mod utils;
use crate::application::print::print_audio_and_video;
use crate::config::{Config, FramePass};
use crate::error::KintaroError;
use crate::realtime::gui::GuiRepaintSignal;
use crate::realtime::RealTimeState;
use crate::renderable::RenderableConfig;
use colored::*;
use cradle::prelude::*;
use rodio::OutputStream;
use std::collections::HashMap;
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
    /// visual data
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

pub fn audios_and_visuals_from_frame_passes(
    frame_passes: &Vec<FramePass>,
) -> Result<(Vec<Audio>, VisualsMap), Error> {
    let mut visuals_map: VisualsMap = HashMap::new();
    let mut audios: Vec<Audio> = vec![];

    for c in frame_passes.iter().flat_map(|c| &c.renderables) {
        if let RenderableConfig::EventStreams(e) = c {
            let result = get_audiovisual_data(&e.socool_path)?;

            let (a, v) = split_audio_visual(result);
            audios.push(a);
            visuals_map.insert(e.socool_path.to_string(), v);
        }
    }

    Ok((audios, visuals_map))
}

pub fn run(filename: &str, config: Config<'static>) -> Result<(), KintaroError> {
    // println!("preparing for audiovisualization: {}", &filename);
    // let (audios, visuals_map) = audios_and_visuals_from_frame_passes(&config.frame_passes)?;

    // let mut audio: Option<Vec<u8>> = None;
    // let _stream: OutputStream;
    // let mut stream_handle: Option<rodio::Sink> = None;
    // if audios.len() > 0 {
    // let a = sum_all_waveforms(audios);
    // let (_s, s_h) = crate::audio::setup_audio(&config, &a);
    // audio = Some(a);
    // _stream = _s;
    // stream_handle = Some(s_h);
    // }

    if std::env::args().any(|x| x == "--print") {
        print_audio_and_video(config)?;
    } else {
        println!("****REALTIME****");
        // realtime(config, visuals_map, stream_handle)?;
    }
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

fn realtime(
    mut config: Config<'static>,
    visuals_map: VisualsMap,
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
        Some(repaint_signal),
        visuals_map,
        stream_handles,
    )?;

    state.play();

    event_loop.run(move |event, _, control_flow| {
        #[allow(unused_assignments)]
        if let Some(ref mut controls) = state.controls {
            controls.platform.handle_event(&event);
        }
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
