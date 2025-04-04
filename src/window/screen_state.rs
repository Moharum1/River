use std::sync::Arc;
use wgpu::{Device, PowerPreference, Queue, Surface};
use winit::dpi::PhysicalSize;
use winit::window::Window;
use pollster::block_on;

pub struct WindowState<'window>{
    surface : Surface<'window>,
    device  : Device,
    queue   : Queue,
    config  : wgpu::SurfaceConfiguration,
    size    : PhysicalSize<u32>,
}

impl<'window> WindowState<'window>{

    // Create A new Window used for WGPU applications
    pub fn new(window : Arc<Window>) -> WindowState<'window>{

        let instance = wgpu::Instance::new(
            &wgpu::InstanceDescriptor{
                ..Default::default()
            }
        );

        let size = window
            .inner_size();

        let surface =
            instance.create_surface(
                Arc::clone(&window)
            ).unwrap();

        let (device, queue, adapter) = block_on( async {
            let adapter = instance.request_adapter(
                &wgpu::RequestAdapterOptionsBase {
                    power_preference: PowerPreference::HighPerformance,
                    force_fallback_adapter: false,
                    compatible_surface: Some(&surface),
                }).await.unwrap();

            let (device, queue) = adapter.request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: Default::default(),
                    required_limits: Default::default(),
                    memory_hints: Default::default(),
                },
                None
            ).await.unwrap();
            (device, queue, adapter)
        });

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage       : wgpu::TextureUsages::RENDER_ATTACHMENT,
            format      : surface_format,
            width       : size.width,
            height      : size.height,
            present_mode: Default::default(),
            desired_maximum_frame_latency: 2,
            alpha_mode  : Default::default(),
            view_formats: vec![],
        };

        WindowState{
            surface,
            device,
            queue,
            config,
            size,
        }
    }
}