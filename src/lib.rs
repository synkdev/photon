pub mod helpers;

use winit::{
	event::*,
	window::Window,
};

/// Main struct for Glixel
pub struct Glixel {
	/// Instance
	pub instance: wgpu::Instance,
	/// Adapter
	pub adapter: wgpu::Adapter,
	/// Surface
	pub surface: wgpu::Surface,
	/// Device
	pub device: wgpu::Device,
	/// Queue
	pub queue: wgpu::Queue,
	/// Surface Configuration
	pub config: wgpu::SurfaceConfiguration,
	/// Window size
	pub size: winit::dpi::PhysicalSize<u32>,
	/// Clear color
	pub clear_color: wgpu::Color,
	/// Winit Window
	pub window: Window,
}

impl Glixel {
	pub async fn new(window: Window) -> Self {}

	pub fn window(&self) -> &Window {
		&self.window
	}

	pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
		if new_size.width > 0 && new_size.height > 0 {
			self.size = new_size;
			self.config.width = new_size.width;
			self.config.height = new_size.height;
			self.surface.configure(&self.device, &self.config);
		}
	}

	pub fn input(&mut self, event: &WindowEvent) -> bool {
		match event {
			WindowEvent::CursorMoved { position, .. } => {
				self.clear_color = wgpu::Color {
					r: position.x as f64 / self.size.width as f64,
					g: position.y as f64 / self.size.height as f64,
					b: 1.0,
					a: 1.0,
				};
				true
			}
			_ => false,
		}
	}

	pub fn update(&mut self) {}

	pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
		let output = self.surface.get_current_texture()?;
		let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
		let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
			label: Some("render_encoder"),
		});

		{
			let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
				label: Some("render_pass"),
				color_attachments: &[Some(wgpu::RenderPassColorAttachment {
					view: &view,
					resolve_target: None,
					ops: wgpu::Operations {
						load: wgpu::LoadOp::Clear(self.clear_color),
						store: wgpu::StoreOp::Store,
					},
				})],
				depth_stencil_attachment: None,
				..Default::default()
			});
		}
		self.queue.submit(std::iter::once(encoder.finish()));
		output.present();

		Ok(())
	}
}
