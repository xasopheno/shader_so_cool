// use crate::config::Config;
// use crate::error::KintaroError;
// use crate::realtime::gui::GuiRepaintSignal;
// use crate::realtime::RealTimeState;
// use winit::{dpi::PhysicalSize, event::*, event_loop::ControlFlow, window::WindowBuilder};

// pub fn realtime(mut config: Config<'static>) -> Result<(), KintaroError> {
// env_logger::init();
// let title = env!("CARGO_PKG_NAME");
// let event_loop = winit::event_loop::EventLoop::with_user_event();
// let window = WindowBuilder::new()
// .with_inner_size(winit::dpi::PhysicalSize {
// width: config.window_size.0,
// height: config.window_size.1,
// })
// .with_transparent(false)
// .with_title(title)
// .build(&event_loop)
// .expect("Unable to create window");

// let repaint_signal = std::sync::Arc::new(GuiRepaintSignal(std::sync::Mutex::new(
// event_loop.create_proxy(),
// )));

// let mut state = RealTimeState::init(&window, &mut config, Some(repaint_signal))?;

// state.play();

// event_loop.run(move |event, _, control_flow| {
// #[allow(unused_assignments)]
// if let Some(ref mut controls) = state.controls {
// controls.platform.handle_event(&event);
// }
// match state.listen_for_new(&config) {
// _ => {}
// }

// match event {
// Event::MainEventsCleared => window.request_redraw(),
// Event::DeviceEvent { ref event, .. } => {
// state.input(event);
// }
// Event::WindowEvent {
// ref event,
// window_id,
// } if window_id == window.id() => {
// // dbg!(event);
// match event {
// WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
// WindowEvent::KeyboardInput { input, .. } => match input {
// KeyboardInput {
// state: ElementState::Pressed,
// virtual_keycode: Some(VirtualKeyCode::Escape),
// ..
// } => {
// *control_flow = ControlFlow::Exit;
// }
// _ => state.keyboard_input(event),
// },
// WindowEvent::Resized(physical_size) => {
// state.resize(*physical_size);
// }
// WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
// state.resize(**new_inner_size);
// }
// _ => {}
// }
// }

// Event::RedrawRequested(_) => {
// match state.render(&window) {
// Ok(_) => {}
// // Recreate the swap_chain if lost
// Err(KintaroError::WgpuSurfaceError(wgpu::SurfaceError::Lost)) => {
// state.resize(PhysicalSize {
// width: state.size.0,
// height: state.size.1,
// })
// }
// // The system is out of memory, we should probably quit
// Err(KintaroError::WgpuSurfaceError(wgpu::SurfaceError::OutOfMemory)) => {
// *control_flow = ControlFlow::Exit
// }
// // All other errors (Outdated, Timeout) should be resolved by the next frame
// Err(e) => eprintln!("{:?}", e),
// };

// *control_flow = ControlFlow::Poll;
// }
// _ => {}
// }
// });
// #[allow(unreachable_code)]
// Ok(())
// }
