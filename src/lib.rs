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

#[derive(Debug)]
pub enum PixelOrient {
	Vertical,
	Horizontal
}

/// Struct for pixel values representing coordinates in pixels.
///
/// Used to represent x and y coordinates in pixels.
#[derive(Debug)]
pub struct Pixel {
	pub value: f32,
	pub orient: PixelOrient
}

pub enum ShapeType {
	Rectangle,
	Circle,
}

#[derive(Debug)]
#[repr(C)]
pub struct Shape {
	pub shape_type: u32,
	pub position: [f32; 2],
	pub size: [f32; 2],
	pub radius: f32,
	pub color: [f32; 4],
}

impl Pixel {
	/// Constructor method to create a new Pixel instance.
	///
	/// # Arguments
	///
	/// * `value` - The value in pixels.
	/// * `orient` - The orientation (height or width).
	///
	/// # Example
	///
	/// ```
	/// use photon::Pixel;
	///
	/// let pixel = Pixel::new(10.0, PixelOrient::Horizontal);
	/// ```
	pub fn new(value: f32, orient: PixelOrient) -> Self {
		Pixel { value, orient }
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
	/// An f32 value of the NDC coordinate.
	///
	/// # Example
	///
	/// ```
	/// use photon::Pixel;
	///
	/// let pixel = Pixel::new(100.0, PixelOrient::Horizontal);
	/// let window_size = (800.0, 600.0);
	/// let ndc_pixel = pixel.to_ndc(window_size);
	/// ```
	pub fn to_ndc(&self, window_size: (f32, f32)) -> f32 {
		match self.orient {
			PixelOrient::Horizontal => {
				(self.value * 2.0) / window_size.0 - 1.0
			}
			PixelOrient::Vertical => {
				(self.value * 2.0) / window_size.1
			}
		}
	}
}

impl Shape {
	pub fn new(
		size: (f32, f32),
		position: (f32, f32),
		color: [f32; 4],
		radius: f32,
		shape_type: ShapeType,
	) -> Self {
		let mut shape_type_index;
		match shape_type {
			ShapeType::Rectangle => shape_type_index = 0,
			ShapeType::Circle => shape_type_index = 1,
		}
		Self {
			size: [size.0, size.1],
			position: [position.0, position.1],
			shape_type: shape_type_index,
			radius,
			color
		}
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
