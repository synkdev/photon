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

/// Struct for pixel values representing coordinates in pixels.
///
/// Used to represent x and y coordinates in pixels.
#[derive(Debug)]
pub struct Pixel {
	pub x: f32,
	pub y: f32,
}

impl Pixel {
	/// Constructor method to create a new Pixel instance.
	///
	/// # Arguments
	///
	/// * `x` - The x-coordinate value in pixels.
	/// * `y` - The y-coordinate value in pixels.
	///
	/// # Example
	///
	/// ```
	/// use photon::Pixel;
	///
	/// let pixel = Pixel::new(10.0, 20.0);
	/// ```
	pub fn new(x: f32, y: f32) -> Self {
		Pixel { x, y }
	}

	/// Method to convert Pixel coordinates to Normalized Device Coordinates (NDC).
	///
	/// Converts the pixel coordinates to NDC using the provided window size.
	///
	/// # Arguments
	///
	/// * `window_size` - A tuple representing the window size (width, height) in pixels.
	///
	/// # Returns
	///
	/// A new Pixel instance with the coordinates converted to NDC.
	///
	/// # Example
	///
	/// ```
	/// use photon::Pixel;
	///
	/// let pixel = Pixel::new(100.0, 200.0);
	/// let window_size = (800.0, 600.0);
	/// let ndc_pixel = pixel.to_ndc(window_size);
	/// ```
	pub fn to_ndc(&self, window_size: (f32, f32)) -> Self {
		let ndc_x = (self.x * 2.0) / window_size.0 - 1.0;
		let ndc_y = 1.0 - (self.y * 2.0) / window_size.1;
		Pixel { x: ndc_x, y: ndc_y }
	}
}

impl Photon {
	/// Creates a new instance of `Photon`.
	///
	/// # Arguments
	///
	/// * `device` - Arc-wrapped wgpu::Device for rendering.
	/// * `queue` - Arc-wrapped wgpu::Queue for rendering commands.
	/// * `texture_format` - wgpu::TextureFormat used in rendering.
	///
	/// # Returns
	///
	/// A new `Photon` instance.
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

	/// Encodes rendering commands based on the provided render pass descriptor.
	///
	/// # Arguments
	///
	/// * `render_pass_desc` - A reference to a wgpu::RenderPassDescriptor.
	///
	/// This method sets up rendering commands based on the provided descriptor
	/// and submits them to the rendering queue.
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
