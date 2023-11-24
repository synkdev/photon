use gleam::WgpuInstance;
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
	let mut instance = WgpuInstance::new(window).await;

	event_loop.run(move |event, _, control_flow| {
		match event {
			Event::WindowEvent { ref event, window_id } if window_id == instance.window.id() => {
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
						instance.resize(*phys_size);
					}
					WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
						instance.resize(**new_inner_size);
					}
					_ => {}
				}
			}
			_ => {}
		}
	});
}

fn main() {
	pollster::block_on(run());
}

