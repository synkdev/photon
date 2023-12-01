use winit::window::Window;

/// Simple state struct for WGPU. You can ship your own implementation and just use
/// the Photon struct for rendering, but this just gets rid of a lot of boilerplate
/// code. All fields are public.
pub struct WgpuState {
	/// Adapter
	pub adapter: wgpu::Adapter,
	/// Surface
	pub surface: wgpu::Surface,
	/// Device
	pub device: wgpu::Device,
	/// Queue
	pub queue: wgpu::Queue,
	/// Window size
	pub size: winit::dpi::PhysicalSize<u32>,
}

impl WgpuState {
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
