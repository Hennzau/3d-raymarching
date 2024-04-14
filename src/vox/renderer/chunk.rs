use glam::{U16Vec4, UVec3};
use wgpu::{Buffer, Device};
use wgpu::util::DeviceExt;
use crate::vox::renderer::terrain::TerrainVertex;

pub struct ChunkRenderData {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
}

impl ChunkRenderData {
    pub fn new(device: &Device) -> Self {
        let vertex_data: Vec<TerrainVertex> = Vec::from([
            TerrainVertex::new(UVec3::new(0, 0, 0), U16Vec4::new(255, 0, 0, 1)),
            TerrainVertex::new(UVec3::new(1, 0, 0), U16Vec4::new(255, 0, 0, 1)),
            TerrainVertex::new(UVec3::new(0, 1, 0), U16Vec4::new(255, 0, 0, 1)),
            TerrainVertex::new(UVec3::new(1, 1, 0), U16Vec4::new(255, 0, 0, 1)),
            TerrainVertex::new(UVec3::new(0, 0, 1), U16Vec4::new(255, 0, 0, 1)),
            TerrainVertex::new(UVec3::new(1, 0, 1), U16Vec4::new(255, 0, 0, 1)),
            TerrainVertex::new(UVec3::new(0, 1, 1), U16Vec4::new(255, 0, 0, 1)),
            TerrainVertex::new(UVec3::new(1, 1, 1), U16Vec4::new(255, 0, 0, 1)),
        ]);

        let index_data: Vec<u16> = Vec::from([
            0, 1, 2, 1, 2, 3,
            4, 5, 6, 5, 6, 7,
            0, 1, 4, 1, 4, 5,
        ]);

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Chunk Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertex_data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Chunk Index Buffer"),
            contents: bytemuck::cast_slice(&index_data),
            usage: wgpu::BufferUsages::INDEX,
        });

        return Self {
            vertex_buffer,
            index_buffer,
        };
    }
}