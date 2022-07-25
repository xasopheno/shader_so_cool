use crate::config::Config;
use crate::error::KintaroError;
use crate::realtime::gui::GuiRepaintSignal;
use crate::realtime::RealTimeState;
use crate::renderable::ToRenderable;
use crossbeam_channel::{bounded, Receiver, Sender};
use notify::{
    event::AccessKind, event::AccessMode, event::EventKind, event::ModifyKind, RecommendedWatcher,
    RecursiveMode, Watcher,
};
use std::path::Path;
use std::sync::{Arc, Mutex};
use weresocool::core::generation::parsed_to_render::{RenderReturn, RenderType};
use weresocool::core::interpretable::{InputType, Interpretable};
use weresocool::core::manager::{RenderManager, VisEvent};
use weresocool::core::portaudio::real_time_render_manager;
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

    let filename = config.socool_path;

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
    render_manager.lock().unwrap().pause();

    let mut state = RealTimeState::init(
        &window,
        &mut config,
        Some(repaint_signal),
        Some(rx),
        render_manager,
    )?;

    let mut watchable_paths: Vec<String> = config
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

    watchable_paths.push(filename.to_string());

    stream.start().unwrap();
    state.play();

    // let time = std::time::SystemTime::now();
    // let mut frames: u64 = 0;

    let (tx, rx) = bounded(1);
    let mut socool_watcher = RecommendedWatcher::new(tx).unwrap();

    watchable_paths.iter().for_each(|path| {
        socool_watcher
            .watch(Path::new(path).as_ref(), RecursiveMode::Recursive)
            .unwrap();
    });

    event_loop.run(move |event, _, control_flow| {
        if let Ok(event) = rx.try_recv() {
            // println!("{:?}", event);
            match event {
                Ok(notify::Event {
                    kind:
                        EventKind::Access(AccessKind::Close(AccessMode::Write))
                        | EventKind::Modify(ModifyKind::Data { .. }),
                    ..
                }) => {
                    println!("updated");
                    state.push_composition(&config).unwrap();
                }
                _ => {
                    // dbg!(event);
                }
            }
        }

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
