use photon::helpers::WgpuState;
use winit::{
	event::*,
	event_loop::{
		ControlFlow,
		EventLoop,
	},
	window::WindowBuilder,
};

pub struct RenderInfo<'a> {
	pub device: &'a wgpu::Device,
	pub surface: &'a wgpu::Surface,
	pub config: &'a wgpu::SurfaceConfiguration,
	pub queue: &'a wgpu::Queue,
}

fn render(render_info: RenderInfo, photon: &mut photon::Photon) {
	let surface = render_info.surface;
	let device = render_info.device;
	let config = render_info.config;
	let frame = match surface.get_current_texture() {
		Ok(frame) => frame,
		Err(_) => {
			surface.configure(device, config);
			surface.get_current_texture().expect("Failed to acquire next surface texture!")
		}
	};

	let texture_view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

	let desc = wgpu::RenderPassDescriptor {
		label: None,
		color_attachments: &[Some(wgpu::RenderPassColorAttachment {
			view: &texture_view,
			resolve_target: None,
			ops: wgpu::Operations {
				load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
				store: wgpu::StoreOp::Store,
			},
		})],
		depth_stencil_attachment: None,
		..Default::default()
	};

	photon.render_encode(&desc);

	frame.present();
}

async fn run() {
	env_logger::init();
	let event_loop = EventLoop::new();
	let window = WindowBuilder::new().with_title("Hello World").build(&event_loop).unwrap();
	let state = WgpuState::new(&window).await;
	let surface = state.surface;
	let device = std::sync::Arc::new(state.device);
	let size = state.size;
	let adapter = state.adapter;
	let queue = std::sync::Arc::new(state.queue);

	let surface_caps = surface.get_capabilities(&adapter);
	let surface_format = surface_caps
		.formats
		.iter()
		.copied()
		.find(|f| f.is_srgb())
		.unwrap_or(surface_caps.formats[0]);
	let mut config = wgpu::SurfaceConfiguration {
		usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
		format: surface_format,
		width: size.width,
		height: size.height,
		present_mode: wgpu::PresentMode::Fifo,
		alpha_mode: wgpu::CompositeAlphaMode::Auto,
		view_formats: vec![],
	};
	surface.configure(&device, &config);

	let mut photon = photon::Photon::new(device.clone(), queue.clone(), config.format);

	let pixel_size =
		photon::Pixel::new(256.0, photon::PixelOrient::Horizontal).to_ndc((size.width as f32, size.height as f32));

	println!("{:#?}", pixel_size);

	event_loop.run(move |event, _, control_flow| {
		match event {
			Event::WindowEvent { ref event, window_id } if window_id == window.id() => {
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
					WindowEvent::Resized(_) => {
						config.width = size.width.max(1);
						config.height = size.height.max(1);
						surface.configure(&device, &config);
						window.request_redraw();
					}
					WindowEvent::ScaleFactorChanged { .. } => {
						config.width = size.width.max(1);
						config.height = size.height.max(1);
						surface.configure(&device, &config);
						window.request_redraw();
					}
					_ => {}
				}
			}
			Event::RedrawRequested(window_id) if window_id == window.id() => {
				render(
					RenderInfo {
						device: &device,
						surface: &surface,
						config: &config,
						queue: &queue,
					},
					&mut photon,
				)
			}
			Event::MainEventsCleared => {
				window.request_redraw();
			}
			_ => {}
		}
	});
}

fn main() {
	pollster::block_on(run());
}
