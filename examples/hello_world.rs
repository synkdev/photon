use gleam::WgpuState;
use winit::{
	event::*,
	event_loop::{
		ControlFlow,
		EventLoop,
	},
	window::WindowBuilder,
};

async fn run() {
	env_logger::init();
	let event_loop = EventLoop::new();
	let window = WindowBuilder::new().build(&event_loop).unwrap();
	let mut state = WgpuState::new(window).await;

	event_loop.run(move |event, _, control_flow| {
		match event {
			Event::WindowEvent { ref event, window_id } if window_id == state.window.id() => {
				match event {
					WindowEvent::CloseRequested
					| WindowEvent::KeyboardInput {
						input:
							KeyboardInput {
								state: ElementState::Pressed,
								virtual_keycode: Some(VirtualKeyCode::Q),
								..
							},
						..
					} => *control_flow = ControlFlow::Exit,
					WindowEvent::Resized(phys_size) => {
						state.resize(*phys_size);
					}
					WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
						state.resize(**new_inner_size);
					}
					_ => {}
				}
			}
			Event::RedrawRequested(window_id) if window_id == state.window().id() => {
				state.update();
				match state.render() {
					Ok(_) => {}
					Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
					Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
					Err(e) => eprintln!("{:?}", e),
				}
			}
			Event::MainEventsCleared => {
				state.window().request_redraw();
			}
			_ => {}
		}
	});
}

fn main() {
	pollster::block_on(run());
}
