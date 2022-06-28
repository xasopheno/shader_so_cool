use crate::clock::Clock;
use crate::config::Config;
use crate::error::KintaroError;
use crate::realtime::gui::GuiRepaintSignal;
use crate::realtime::RealTimeState;
// use crate::realtime::Watcher;
use crate::renderable::ToRenderable;
use crossbeam_channel::{Receiver, Sender};
use notify::{
    event::AccessKind, event::AccessMode, event::EventKind, event::ModifyKind, RecommendedWatcher,
    RecursiveMode, Watcher,
};
use std::collections::HashSet;
use std::path::Path;
use std::sync::mpsc::channel;
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

    // let watchers = Watcher::init(watchable_paths.clone())?;
    // println!("Created {} watchers", watchable_paths.len());

    stream.start().unwrap();
    state.play();

    let time = std::time::SystemTime::now();
    let mut frames: u64 = 0;

    let (tx, rx) = channel();
    // let (output_tx, output_rx) = channel();
    let mut socool_watcher = RecommendedWatcher::new(tx).unwrap();

    watchable_paths.iter().for_each(|path| {
        socool_watcher
            .watch(Path::new(path).as_ref(), RecursiveMode::Recursive)
            .unwrap();
    });

    event_loop.run(move |event, _, control_flow| {
        // if watchers.receiver.try_recv().is_ok() {
        let mut x = || {
            std::thread::sleep(std::time::Duration::from_millis(100));
            state.push_composition(&config).unwrap();
            state.clock.reset();
            state.play();
        };
        // };

        if let Ok(event) = rx.try_recv() {
            // println!("{:?}", event);
            match event {
                Ok(notify::Event {
                    kind: EventKind::Access(AccessKind::Close(AccessMode::Write)),
                    ..
                }) => {
                    println!("updated");
                    x();
                    // output_tx.send(true).expect("oh no watcher can't send");
                }
                Ok(notify::Event {
                    kind: EventKind::Modify(ModifyKind::Data { .. }),
                    ..
                }) => {
                    println!("updated");
                    x();
                    // output_tx.send(true).expect("oh no! watcher can't send!");
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
                frames += 1;
                if frames % 100 == 0 {
                    let elapsed = time.elapsed().unwrap();
                    println!("\r{}fps", (frames / (elapsed.as_secs() + 1)),);
                };
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

// pub fn watch(path_strings: Vec<String>) -> Result<Receiver<bool>, notify::Error> {
// let path_strings: Vec<String> = path_strings
// .into_iter()
// .collect::<HashSet<_>>()
// .into_iter()
// .collect();
// let (tx, rx) = channel();
// let (output_tx, output_rx) = channel();
// let mut socool_watcher = RecommendedWatcher::new(tx).unwrap();

// std::thread::spawn(move || -> Result<(), notify::Error> {
// path_strings.iter().for_each(|path| {
// socool_watcher
// .watch(Path::new(path).as_ref(), RecursiveMode::Recursive)
// .unwrap();
// });

// loop {
// if let Ok(event) = rx.try_recv() {
// // println!("{:?}", event);
// match event {
// Ok(notify::Event {
// kind: EventKind::Access(AccessKind::Close(AccessMode::Write)),
// ..
// }) => {
// println!("updated");
// output_tx.send(true).expect("oh no watcher can't send");
// }
// Ok(notify::Event {
// kind: EventKind::Modify(ModifyKind::Data { .. }),
// ..
// }) => {
// println!("updated");
// output_tx.send(true).expect("oh no! watcher can't send!");
// }
// _ => {
// // dbg!(event);
// }
// }
// }
// }
// // Ok(())
// });

// Ok(output_rx)
// }
