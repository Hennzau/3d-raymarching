use wgpu::{Device, Surface, Adapter, Queue, SurfaceConfiguration, util::DeviceExt, TextureView, Texture, Sampler, TextureFormat};

use bytemuck::{
    Pod,
    Zeroable,
};

use glam::Mat4;

use crate::vox::renderer::{
    chunk::ChunkRenderData,
    terrain::TerrainRenderer,
};

pub mod terrain;
pub mod chunk;

pub struct VoxRenderer {
    terrain_renderer: TerrainRenderer,
    depth_texture: (Texture, TextureView, Sampler),

    chunk_render_data: Vec<ChunkRenderData>,
}

impl VoxRenderer {
    pub fn new(device: &Device, config: &SurfaceConfiguration, surface: &Surface, adapter: &Adapter) -> Self {
        return Self {
            terrain_renderer: TerrainRenderer::new(device, surface, adapter),
            depth_texture: Self::create_depth_texture(device, (config.width, config.height)),
            chunk_render_data: Vec::from([ChunkRenderData::new(device)]),
        };
    }

    fn create_depth_texture(device: &Device, size: (u32, u32)) -> (Texture, TextureView, Sampler) {
        let size = wgpu::Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: 1,
        };

        let desc = wgpu::TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };

        let texture = device.create_texture(&desc);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(
            &wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Nearest,
                compare: Some(wgpu::CompareFunction::LessEqual),
                lod_min_clamp: 0.0,
                lod_max_clamp: 100.0,
                ..Default::default()
            }
        );

        return (texture, view, sampler);
    }

    pub fn render(&self, device: &Device, surface: &Surface, queue: &Queue) {
        let frame = surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: None,
            });
        {
            let mut render_pass =
                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self.depth_texture.1,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

            render_pass.set_pipeline(&self.terrain_renderer.render_pipeline);
            render_pass.set_bind_group(0, &self.terrain_renderer.bind_group, &[]);

            for chunk_render_data in &self.chunk_render_data {
                render_pass.set_vertex_buffer(0, chunk_render_data.vertex_buffer.slice(..));
                render_pass.set_index_buffer(chunk_render_data.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

                render_pass.draw_indexed(0..chunk_render_data.index_count as u32, 0, 0..1);
            };
        }

        queue.submit(Some(encoder.finish()));
        frame.present();
    }

    pub fn process_resize(&mut self, new_size: (u32, u32), device: &Device, queue: &Queue, projection_view_matrix: Mat4) {
        self.update_projection_view_uniform(queue, projection_view_matrix);

        self.depth_texture = Self::create_depth_texture(device, (new_size.0, new_size.1));
    }

    pub fn update_projection_view_uniform(&mut self, queue: &Queue, projection_view_matrix: Mat4) {
        self.terrain_renderer.update_projection_view_uniform(queue, projection_view_matrix);
    }
}