pub mod helpers;

use std::sync::Arc;

/// Main struct for Photon
pub struct Photon {
	/// Device
	pub device: Arc<wgpu::Device>,
	/// Queue
	pub queue: Arc<wgpu::Queue>,
	/// Render pipeline
	pub render_pipeline: wgpu::RenderPipeline,
}

/// Struct for pixel values
pub struct Pixel {
	x: f32,
	y: f32,
}

impl Pixel {
	pub fn new(x: f32, y: f32, window_size: (f32, f32)) -> Self {
		let ndc_x = (x * 2.0) / window_size.0 - 1.0;
		let ndc_y = 1.0 - (y * 2.0) / window_size.1;
		Pixel { x: ndc_x, y: ndc_y }
	}
}

impl Photon {
	pub fn new(
		device: Arc<wgpu::Device>,
		queue: Arc<wgpu::Queue>,
		texture_format: wgpu::TextureFormat,
	) -> Self {
		let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

		let render_pipeline_layout =
			device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
				label: Some("render_pipeline_layout"),
				bind_group_layouts: &[],
				push_constant_ranges: &[],
			});

		let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: Some("render_pipeline"),
			layout: Some(&render_pipeline_layout),
			vertex: wgpu::VertexState { module: &shader, entry_point: "vs_main", buffers: &[] },
			fragment: Some(wgpu::FragmentState {
				module: &shader,
				entry_point: "fs_main",
				targets: &[Some(wgpu::ColorTargetState {
					format: texture_format,
					blend: Some(wgpu::BlendState {
						color: wgpu::BlendComponent::REPLACE,
						alpha: wgpu::BlendComponent::REPLACE,
					}),
					write_mask: wgpu::ColorWrites::ALL,
				})],
			}),
			primitive: wgpu::PrimitiveState {
				topology: wgpu::PrimitiveTopology::TriangleList,
				strip_index_format: None,
				front_face: wgpu::FrontFace::Ccw,
				cull_mode: Some(wgpu::Face::Back),
				polygon_mode: wgpu::PolygonMode::Fill,
				unclipped_depth: false,
				conservative: false,
			},
			depth_stencil: None,
			multisample: wgpu::MultisampleState {
				count: 1,
				mask: !0,
				alpha_to_coverage_enabled: false,
			},
			multiview: None,
		});
		Self { device, queue, render_pipeline }
	}

	pub fn render_encode(&mut self, render_pass_desc: &wgpu::RenderPassDescriptor) {
		let device = &self.device;
		let queue = &self.queue;

		let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
			label: Some("render_encoder"),
		});

		let mut render_pass = encoder.begin_render_pass(render_pass_desc);

		render_pass.set_pipeline(&self.render_pipeline);

		render_pass.draw(0..3, 0..1);

		drop(render_pass);
		queue.submit(Some(encoder.finish()));
	}
}
