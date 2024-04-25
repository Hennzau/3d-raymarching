use wgpu::{
    LoadOp,
    Operations,
    RenderPassColorAttachment,
    RenderPassDescriptor,
    util::DeviceExt
};

use crate::{
    logic::Logic,
    WGPUBackend
};

use crate::logic::{
    LogicState,
    play::PipelineType
};

pub mod pipeline;
pub mod rasterizer;
pub mod ray_marcher;

pub struct Renderer {
    rasterizer: rasterizer::TestRasterizer,
    ray_marcher: ray_marcher::TestRayMarcher,
}

impl Renderer {
    pub fn new(wgpu_backend: &WGPUBackend, logic: &Logic) -> Self {
        let rasterizer = rasterizer::TestRasterizer::new(wgpu_backend, &logic.play);
        let ray_marcher = ray_marcher::TestRayMarcher::new(wgpu_backend, &logic.play);

        return Self {
            rasterizer,
            ray_marcher,
        };
    }

    pub fn update(&mut self, wgpu_backend: &WGPUBackend, logic: &Logic) {
        self.rasterizer.update(wgpu_backend, &logic.play);
        self.ray_marcher.update(wgpu_backend, &logic.play);
    }

    pub fn process_resize(&mut self, wgpu_backend: &WGPUBackend, logic: &Logic) {
        self.ray_marcher.process_resize(wgpu_backend, &logic.play);
    }

    pub fn render(&self, wgpu_backend: &WGPUBackend, logic: &Logic) {
        let frame = wgpu_backend.surface.get_current_texture().expect("Failed to acquire next swap chain texture");
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = wgpu_backend.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: None,
        });

        {
            let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            match logic.state {
                LogicState::Playing => {
                    if logic.play.pipeline == PipelineType::TestRasterizer {
                        self.rasterizer.render(&mut pass);
                    } else {
                        self.ray_marcher.render(&mut pass);
                    }
                },
                LogicState::Menu => {},
            }
        }

        wgpu_backend.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}