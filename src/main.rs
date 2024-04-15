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
use winit::dpi::LogicalSize;

use crate::vox::VoxLogic;
use crate::renderer::VoxRenderer;

mod vox;
mod renderer;

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

    let config = surface
        .get_default_config(&adapter, size.width, size.height)
        .unwrap();
    surface.configure(&device, &config);

    return (instance, surface, config, adapter, device, queue);
}

fn main() {
    let mut event_loop = EventLoop::new().unwrap();
    let builder = WindowBuilder::new();
    let window = builder
        .with_title("Voxel Engine")
        .with_inner_size(LogicalSize::new(1280, 720))
        .build(&event_loop).unwrap();

    let (_instance, surface, mut config, adapter, device, queue) = pollster::block_on(build_backend(&window));

    let mut vox = VoxLogic::new(config.width as f32 / config.height as f32);
    let mut renderer = VoxRenderer::new(&device, &config, &surface, &adapter, &vox.world);

    'main: loop {
        let timeout = Some(Duration::ZERO);
        let status = event_loop.pump_events(timeout, |event, target| {
            match event {
                Event::AboutToWait => window.request_redraw(),
                Event::WindowEvent {
                    event,
                    ..
                } => {
                    match event {
                        WindowEvent::Resized(new_size) => {
                            config.width = new_size.width.max(1);
                            config.height = new_size.height.max(1);

                            surface.configure(&device, &config);

                            window.request_redraw();

                            vox.process_resize((config.width, config.height));
                            renderer.process_resize((config.width, config.height), &device, &queue, vox.camera.projection_view_matrix);
                        }
                        WindowEvent::CloseRequested => target.exit(),
                        WindowEvent::RedrawRequested => renderer.render(&device, &surface, &queue),
                        WindowEvent::KeyboardInput {
                            event,
                            ..
                        } => vox.process_keyboard(event),
                        _ => {}
                    }
                }
                _ => {}
            }
        });

        if let PumpStatus::Exit(_) = status {
            break 'main;
        }

        vox.update(Duration::from_millis(16).as_secs_f32());
        renderer.update_projection_view_uniform(&queue, vox.camera.projection_view_matrix);

        sleep(Duration::from_millis(16)); // At the moment we just put everything at 60 ticks/per_second
    }
}