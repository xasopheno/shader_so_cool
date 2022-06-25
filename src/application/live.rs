use crate::config::Config;
use crate::error::KintaroError;
use crate::realtime::gui::GuiRepaintSignal;
use crate::realtime::RealTimeState;
use crate::realtime::Watcher;
use crate::renderable::ToRenderable;
use crossbeam_channel::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use weresocool::generation::parsed_to_render::{RenderReturn, RenderType};
use weresocool::interpretable::{InputType, Interpretable};
use weresocool::manager::{RenderManager, VisEvent};
use weresocool::portaudio::real_time_render_manager;
use weresocool_instrument::renderable::{nf_to_vec_renderable, renderables_to_render_voices};
use winit::{dpi::PhysicalSize, event::*, event_loop::ControlFlow, window::WindowBuilder};

pub fn live(mut config: Config<'static>) -> Result<(), KintaroError> {
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
        .build(&event_loop)
        .expect("Unable to create window");

    let repaint_signal = std::sync::Arc::new(GuiRepaintSignal(std::sync::Mutex::new(
        event_loop.create_proxy(),
    )));

    let (tx, rx): (Sender<VisEvent>, Receiver<VisEvent>) = crossbeam_channel::unbounded();

    let filename = "./kintaro3.socool";

    let (nf, basis, mut table) =
        match InputType::Filename(filename).make(RenderType::NfBasisAndTable, None)? {
            RenderReturn::NfBasisAndTable(nf, basis, table) => (nf, basis, table),
            _ => panic!("Error. Unable to generate NormalForm"),
        };
    let renderables = nf_to_vec_renderable(&nf, &mut table, &basis)?;
    let render_voices = renderables_to_render_voices(renderables);

    let render_manager = Arc::new(Mutex::new(RenderManager::init(
        render_voices,
        Some(tx),
        None,
        false,
    )));
    let mut stream = real_time_render_manager(Arc::clone(&render_manager))?;
    //TODO: Move real_time_render_manager into RealTimeState
    //need to push new ops to render_manager on push
    render_manager.lock().unwrap().pause();

    let mut state = RealTimeState::init(
        &window,
        &mut config,
        Some(repaint_signal),
        Some(rx),
        render_manager,
    )?;

    let watchable_paths: Vec<String> = config
        .frame_passes
        .iter()
        .map(|frame_pass| {
            frame_pass
                .renderables
                .iter()
                .map(|r| r.watchable_paths())
                .flatten()
                .collect::<Vec<String>>()
        })
        .flatten()
        .collect();

    let watchers = Watcher::init(watchable_paths.clone())?;
    println!("Created {} watchers", watchable_paths.len());

    stream.start().unwrap();
    state.play();
    // render_manager.lock().unwrap().play();
    let time = std::time::SystemTime::now();
    let mut frames: u64 = 0;

    event_loop.run(move |event, _, control_flow| {
        frames += 1;
        if (frames % 1000 == 0) {
            let elapsed = time.elapsed().unwrap();
            println!(
                "\r{:?}, {:?}, {:?}",
                (frames / (elapsed.as_secs() + 1)),
                frames,
                elapsed.as_secs() + 1
            );
        }

        if watchers.receiver.try_recv().is_ok() {
            state.push_composition(&config).unwrap();
        };

        #[allow(unused_assignments)]
        if let Some(ref mut controls) = state.controls {
            controls.platform.handle_event(&event);
        }
        match state.listen_for_new(&config) {
            _ => {}
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
