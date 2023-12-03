use winit::window::Window;

/// Simple state struct for WGPU. It encapsulates various WGPU components for rendering.
pub struct WgpuState {
	/// Adapter used for the rendering device.
	pub adapter: wgpu::Adapter,
	/// Surface for rendering.
	pub surface: wgpu::Surface,
	/// Device used for rendering.
	pub device: wgpu::Device,
	/// Queue for sending rendering commands.
	pub queue: wgpu::Queue,
	/// Size of the window.
	pub size: winit::dpi::PhysicalSize<u32>,
}

impl WgpuState {
	/// Creates a new `WgpuState` instance based on the provided `Window`.
	///
	/// # Arguments
	///
	/// * `window` - A reference to a `Window` instance from winit.
	///
	/// # Returns
	///
	/// A new `WgpuState` instance initialized with the necessary WGPU components.
	pub async fn new(window: &Window) -> Self {
		let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
			backends: wgpu::Backends::VULKAN,
			..Default::default()
		});

		let size = window.inner_size();

		let surface = unsafe { instance.create_surface(&window) }.unwrap();

		let adapter = instance
			.request_adapter(&wgpu::RequestAdapterOptionsBase {
				power_preference: wgpu::PowerPreference::HighPerformance,
				force_fallback_adapter: false,
				compatible_surface: Some(&surface),
			})
			.await
			.unwrap();

		let (device, queue) = adapter
			.request_device(
				&wgpu::DeviceDescriptor {
					features: wgpu::Features::empty(),
					limits: wgpu::Limits::default(),
					label: None,
				},
				None,
			)
			.await
			.unwrap();

		Self { size, surface, adapter, device, queue }
	}
}
