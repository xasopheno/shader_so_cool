use crate::config::Config;
use crate::error::KintaroError;
use crate::realtime::gui::GuiRepaintSignal;
use crate::realtime::RealTimeState;
use crossbeam_channel::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use weresocool::generation::{
    json::Op4D,
    parsed_to_render::{RenderReturn, RenderType},
};
use weresocool::interpretable::{InputType, Interpretable};
use weresocool::manager::RenderManager;
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

    let (tx, rx): (Sender<Vec<Op4D>>, Receiver<Vec<Op4D>>) = crossbeam_channel::unbounded();

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
    // TODO: start_paused

    // put channel in RealTimeState
    let mut state = RealTimeState::init(&window, &mut config, Some(repaint_signal))?;

    stream.start().unwrap();
    state.play();
    render_manager.lock().unwrap().play();

    event_loop.run(move |event, _, control_flow| {
        if let Ok(ops) = rx.try_recv() {
            dbg!(ops.len());
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
