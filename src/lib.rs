pub mod helpers;

use std::sync::Arc;
use winit::{
	event::*,
	window::Window,
};

/// Main struct for Glix
pub struct Glix {
	/// Device
	pub device: Arc<wgpu::Device>,
	/// Queue
	pub queue: Arc<wgpu::Queue>,
}

impl Glix {
	pub async fn new(device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>) -> Self {
		Self { device, queue }
	}

	// pub fn update(&mut self) {}
	//
	// pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
	// 	let output = self.surface.get_current_texture()?;
	// 	let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
	// 	let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
	// 		label: Some("render_encoder"),
	// 	});
	//
	// 	{
	// 		let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
	// 			label: Some("render_pass"),
	// 			color_attachments: &[Some(wgpu::RenderPassColorAttachment {
	// 				view: &view,
	// 				resolve_target: None,
	// 				ops: wgpu::Operations {
	// 					load: wgpu::LoadOp::Clear(self.clear_color),
	// 					store: wgpu::StoreOp::Store,
	// 				},
	// 			})],
	// 			depth_stencil_attachment: None,
	// 			..Default::default()
	// 		});
	// 	}
	// 	self.queue.submit(std::iter::once(encoder.finish()));
	// 	output.present();
	//
	// 	Ok(())
	// }
}
