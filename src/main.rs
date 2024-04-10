use std::{
    thread::sleep,
    time::Duration,
};

use wgpu::{
    Adapter,
    Device,
    Instance,
    Queue,
    Surface,
    SurfaceConfiguration,
};

use winit::{
    event::{
        Event,
        WindowEvent,
    },
    event_loop::EventLoop,
    platform::pump_events::{
        EventLoopExtPumpEvents,
        PumpStatus,
    },
    window::{
        Window,
        WindowBuilder,
    },
};

use crate::vox::renderer::VoxRenderer;
use crate::vox::VoxLogic;

mod vox;

async fn build_backend(window: &Window) -> (Instance, Surface, SurfaceConfiguration, Adapter, Device, Queue) {
    let instance = wgpu::Instance::default();

    let surface = instance.create_surface(window).unwrap();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropriate adapter");

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    let mut size = window.inner_size();
    size.width = size.width.max(1);
    size.height = size.height.max(1);

    let mut config = surface
        .get_default_config(&adapter, size.width, size.height)
        .unwrap();
    surface.configure(&device, &config);

    return (instance, surface, config, adapter, device, queue);
}

fn main() {
    let mut event_loop = EventLoop::new().unwrap();
    let builder = WindowBuilder::new();
    let window = builder.build(&event_loop).unwrap();

    let (instance, surface, mut config, adapter, device, queue) = pollster::block_on(build_backend(&window));

    let mut vox = VoxLogic::new();
    let mut renderer = VoxRenderer::new(&device, &config, &surface, &adapter);

    'main: loop {
        let timeout = Some(Duration::ZERO);
        let status = event_loop.pump_events(timeout, |event, target| {
            match event {
                Event::AboutToWait => window.request_redraw(),
                Event::WindowEvent {
                    event: WindowEvent::Resized(new_size),
                    window_id
                } => {
                    config.width = new_size.width.max(1);
                    config.height = new_size.height.max(1);

                    surface.configure(&device, &config);

                    window.request_redraw();
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id
                } => target.exit(),
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    window_id
                } => {
                    renderer.render(&device, &surface, &queue);
                }
                _ => {}
            }
        });

        if let PumpStatus::Exit(exit_code) = status {
            break 'main;
        }

        vox.update(&mut renderer);

        sleep(Duration::from_millis(16)); // At the moment we just put everything at 60 ticks/per_second
    }
}